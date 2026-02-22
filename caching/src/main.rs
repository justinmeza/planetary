use caching::{
    DeleteArgs, GetArgs, GetResult, ModeArgs, ModeResult, ReplicateDeleteArgs, ReplicateSetArgs,
    SetArgs, StatsArgs, StatsResult, DELETE_PROCEDURE, GET_PROCEDURE, MODE_PROCEDURE,
    REPLICATE_DELETE_PROCEDURE, REPLICATE_SET_PROCEDURE, SET_PROCEDURE, STATS_PROCEDURE,
    SYSTEM_ADDRESS, SYSTEM_NAME,
};
use rpc::{server, Request, Response};
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

const MAX_CAPACITY: usize = 10000;
const CLEANUP_INTERVAL: Duration = Duration::from_secs(5);

struct CacheEntry {
    value: String,
    expires_at: Instant,
    version: u64,
}

struct Cache {
    entries: HashMap<String, CacheEntry>,
    lru_order: VecDeque<String>,
    hits: i32,
    misses: i32,
    max_capacity: usize,
    next_version: u64,
    consistency_mode: String,
    own_addr: String,
}

impl Cache {
    fn new(max_capacity: usize, consistency_mode: String, own_addr: String) -> Self {
        Cache {
            entries: HashMap::new(),
            lru_order: VecDeque::new(),
            hits: 0,
            misses: 0,
            max_capacity,
            next_version: 1,
            consistency_mode,
            own_addr,
        }
    }

    fn get(&mut self, key: &str) -> Option<(String, u64)> {
        if let Some(entry) = self.entries.get(key) {
            if entry.expires_at < Instant::now() {
                self.entries.remove(key);
                self.lru_order.retain(|k| k != key);
                self.misses += 1;
                return None;
            }
            let value = entry.value.clone();
            let version = entry.version;
            self.lru_order.retain(|k| k != key);
            self.lru_order.push_front(key.to_string());
            self.hits += 1;
            Some((value, version))
        } else {
            self.misses += 1;
            None
        }
    }

    fn set(&mut self, key: String, value: String, ttl_secs: i32) -> u64 {
        let version = self.next_version;
        self.next_version += 1;
        self.set_with_version(key, value, ttl_secs, version);
        version
    }

    fn set_with_version(&mut self, key: String, value: String, ttl_secs: i32, version: u64) {
        let ttl = if ttl_secs > 0 {
            Duration::from_secs(ttl_secs as u64)
        } else {
            Duration::from_secs(3600)
        };

        // Only apply if version >= current
        if let Some(existing) = self.entries.get(&key) {
            if version < existing.version {
                return;
            }
        }

        let entry = CacheEntry {
            value,
            expires_at: Instant::now() + ttl,
            version,
        };

        self.lru_order.retain(|k| k != &key);

        while self.entries.len() >= self.max_capacity {
            if let Some(evicted_key) = self.lru_order.pop_back() {
                self.entries.remove(&evicted_key);
                println!("Evicted key: {}", evicted_key);
            } else {
                break;
            }
        }

        if version >= self.next_version {
            self.next_version = version + 1;
        }

        self.entries.insert(key.clone(), entry);
        self.lru_order.push_front(key);
    }

    fn delete(&mut self, key: &str) -> u64 {
        let version = self.next_version;
        self.next_version += 1;
        self.entries.remove(key);
        self.lru_order.retain(|k| k != key);
        version
    }

    fn delete_with_version(&mut self, key: &str, version: u64) {
        if let Some(existing) = self.entries.get(key) {
            if version < existing.version {
                return;
            }
        }
        self.entries.remove(key);
        self.lru_order.retain(|k| k != key);
        if version >= self.next_version {
            self.next_version = version + 1;
        }
    }

    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        let expired_keys: Vec<String> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.expires_at < now)
            .map(|(key, _)| key.clone())
            .collect();

        for key in &expired_keys {
            self.entries.remove(key);
            self.lru_order.retain(|k| k != key);
        }

        if !expired_keys.is_empty() {
            println!("Cleaned up {} expired entries", expired_keys.len());
        }
    }
}

async fn get_peers(own_addr: &str) -> Vec<String> {
    let mut peers: Vec<String> = Vec::new();
    let result = discovery::list(SYSTEM_NAME.to_string()).await;
    for s in result.addresses.split(';') {
        if !s.is_empty() && s != own_addr {
            peers.push(s.to_string());
        }
    }
    peers
}

mod handlers {
    use super::*;

    pub async fn get(payload: &str, cache: &mut Cache) -> Response {
        let args = GetArgs::deserialize(payload).expect("Failed to deserialize payload");

        match cache.get(&args.key) {
            Some((value, _version)) => {
                let result = GetResult { value, hit: 1 };
                Response {
                    payload: result.serialize(),
                }
            }
            None => {
                let result = GetResult {
                    value: String::new(),
                    hit: 0,
                };
                Response {
                    payload: result.serialize(),
                }
            }
        }
    }

    pub async fn set(payload: &str, cache: &mut Cache) -> Response {
        let args = SetArgs::deserialize(payload).expect("Failed to deserialize payload");
        let version = cache.set(args.key.clone(), args.value.clone(), args.ttl_secs);
        let mode = cache.consistency_mode.clone();
        let own_addr = cache.own_addr.clone();

        // Replicate based on consistency mode
        match mode.as_str() {
            "eventual" => {
                // Fire-and-forget async replication
                let key = args.key.clone();
                let value = args.value.clone();
                let ttl = args.ttl_secs;
                let ver = version as i32;
                let own = own_addr.clone();
                tokio::spawn(async move {
                    let peers = get_peers(&own).await;
                    for peer in &peers {
                        let _ =
                            caching::replicate_set(peer, key.clone(), value.clone(), ttl, ver)
                                .await;
                    }
                });
            }
            "quorum" => {
                // Wait for 1 peer ack (W=2 of N=3)
                let peers = get_peers(&own_addr).await;
                let mut acks = 0;
                for peer in &peers {
                    if acks >= 1 {
                        break;
                    }
                    let result = caching::replicate_set(
                        peer,
                        args.key.clone(),
                        args.value.clone(),
                        args.ttl_secs,
                        version as i32,
                    )
                    .await;
                    if !result.starts_with("ERROR") {
                        acks += 1;
                    }
                }
            }
            "strong" => {
                // Wait for ALL peer acks (W=N)
                let peers = get_peers(&own_addr).await;
                for peer in &peers {
                    let _ = caching::replicate_set(
                        peer,
                        args.key.clone(),
                        args.value.clone(),
                        args.ttl_secs,
                        version as i32,
                    )
                    .await;
                }
            }
            _ => {}
        }

        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn delete(payload: &str, cache: &mut Cache) -> Response {
        let args = DeleteArgs::deserialize(payload).expect("Failed to deserialize payload");
        let version = cache.delete(&args.key);
        let mode = cache.consistency_mode.clone();
        let own_addr = cache.own_addr.clone();

        // Replicate delete based on consistency mode
        match mode.as_str() {
            "eventual" => {
                let key = args.key.clone();
                let ver = version as i32;
                let own = own_addr.clone();
                tokio::spawn(async move {
                    let peers = get_peers(&own).await;
                    for peer in &peers {
                        let _ =
                            caching::replicate_delete_remote(peer, key.clone(), ver).await;
                    }
                });
            }
            "quorum" => {
                let peers = get_peers(&own_addr).await;
                let mut acks = 0;
                for peer in &peers {
                    if acks >= 1 {
                        break;
                    }
                    let result = caching::replicate_delete_remote(
                        peer,
                        args.key.clone(),
                        version as i32,
                    )
                    .await;
                    if !result.starts_with("ERROR") {
                        acks += 1;
                    }
                }
            }
            "strong" => {
                let peers = get_peers(&own_addr).await;
                for peer in &peers {
                    let _ = caching::replicate_delete_remote(
                        peer,
                        args.key.clone(),
                        version as i32,
                    )
                    .await;
                }
            }
            _ => {}
        }

        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn stats(payload: &str, cache: &mut Cache) -> Response {
        let _args = StatsArgs::deserialize(payload).expect("Failed to deserialize payload");
        let result = StatsResult {
            hits: cache.hits,
            misses: cache.misses,
            size: cache.entries.len() as i32,
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn replicate_set(payload: &str, cache: &mut Cache) -> Response {
        let args =
            ReplicateSetArgs::deserialize(payload).expect("Failed to deserialize payload");
        cache.set_with_version(args.key, args.value, args.ttl_secs, args.version as u64);
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn replicate_delete(payload: &str, cache: &mut Cache) -> Response {
        let args =
            ReplicateDeleteArgs::deserialize(payload).expect("Failed to deserialize payload");
        cache.delete_with_version(&args.key, args.version as u64);
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn mode(payload: &str, cache: &mut Cache) -> Response {
        let args = ModeArgs::deserialize(payload).expect("Failed to deserialize payload");
        if !args.mode.is_empty() {
            // Set mode
            match args.mode.as_str() {
                "eventual" | "quorum" | "strong" => {
                    cache.consistency_mode = args.mode.clone();
                    println!("Consistency mode changed to: {}", args.mode);
                }
                _ => {
                    return Response {
                        payload: ModeResult {
                            mode: format!("ERROR: unknown mode '{}'", args.mode),
                        }
                        .serialize(),
                    };
                }
            }
        }
        let result = ModeResult {
            mode: cache.consistency_mode.clone(),
        };
        Response {
            payload: result.serialize(),
        }
    }
}

async fn request_handler(request: Request, shared_state: Arc<Mutex<Cache>>) -> Response {
    let mut cache = shared_state.lock().await;
    match request.procedure_id {
        GET_PROCEDURE => handlers::get(&request.payload, &mut cache).await,
        SET_PROCEDURE => handlers::set(&request.payload, &mut cache).await,
        DELETE_PROCEDURE => handlers::delete(&request.payload, &mut cache).await,
        STATS_PROCEDURE => handlers::stats(&request.payload, &mut cache).await,
        REPLICATE_SET_PROCEDURE => handlers::replicate_set(&request.payload, &mut cache).await,
        REPLICATE_DELETE_PROCEDURE => {
            handlers::replicate_delete(&request.payload, &mut cache).await
        }
        MODE_PROCEDURE => handlers::mode(&request.payload, &mut cache).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let host = std::env::var("BIND_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let addr = std::env::var("PORT")
        .map(|p| format!("{}:{}", host, p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());

    let consistency_mode =
        std::env::var("CONSISTENCY_MODE").unwrap_or_else(|_| "eventual".to_string());

    let cache = Arc::new(Mutex::new(Cache::new(
        MAX_CAPACITY,
        consistency_mode.clone(),
        addr.clone(),
    )));

    // Background cleanup task for expired entries
    let cleanup_cache = Arc::clone(&cache);
    tokio::spawn(async move {
        loop {
            sleep(CLEANUP_INTERVAL).await;
            cleanup_cache.lock().await.cleanup_expired();
        }
    });

    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    println!(
        "Caching service starting on {} (mode={})",
        addr, consistency_mode
    );

    server::start_server_with_state(
        &addr,
        |request, state| {
            Box::pin(request_handler(request, state))
                as Pin<Box<dyn Future<Output = Response> + Send>>
        },
        cache,
    )
    .await
    .expect("Server crashed");
}
