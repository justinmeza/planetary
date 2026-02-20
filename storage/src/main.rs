mod engine;

use engine::StorageEngine;
use rpc::{server, Request, Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use storage::{
    DeleteArgs, GetArgs, GetPeersArgs, GetPeersResult, GetResult, PutArgs, ReplicateDeleteArgs,
    ReplicatePutArgs, ScanArgs, ScanResult, DELETE_PROCEDURE, GET_PEERS_PROCEDURE, GET_PROCEDURE,
    PUT_PROCEDURE, REPLICATE_DELETE_PROCEDURE, REPLICATE_PUT_PROCEDURE, SCAN_PROCEDURE,
    SYSTEM_ADDRESS, SYSTEM_NAME,
};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

const COMPACTION_INTERVAL: Duration = Duration::from_secs(60);

struct StorageState {
    engine: StorageEngine,
    own_addr: String,
    quorum_w: i32,
    quorum_r: i32,
}

async fn get_peers(own_addr: &str) -> Vec<String> {
    let result = discovery::list(SYSTEM_NAME.to_string()).await;
    result
        .addresses
        .split(';')
        .filter(|s| !s.is_empty() && *s != own_addr)
        .map(|s| s.to_string())
        .collect()
}

mod handlers {
    use super::*;

    pub async fn get(payload: &str, state: &mut StorageState) -> Response {
        let args = GetArgs::deserialize(payload).expect("Failed to deserialize payload");

        let local = state.engine.get_versioned(&args.key);
        let local_value = local.map(|v| v.value.clone()).unwrap_or_default();
        let _local_version = local.map(|v| v.version).unwrap_or(0);
        let local_found = if local.is_some() { 1 } else { 0 };

        // For quorum reads (R > 1), read from peers
        if state.quorum_r > 1 {
            let peers = get_peers(&state.own_addr).await;
            let needed = (state.quorum_r - 1) as usize; // local counts as 1

            let mut best_value = local_value.clone();
            let mut best_found = local_found;

            let mut acks = 0;
            for peer in &peers {
                if acks >= needed {
                    break;
                }
                let result = storage::remote_get(peer, args.key.clone()).await;
                acks += 1;

                if result.found == 1 && best_found == 0 {
                    best_value = result.value.clone();
                    best_found = 1;
                }
            }

            let result = GetResult {
                value: best_value,
                found: best_found,
            };
            return Response {
                payload: result.serialize(),
            };
        }

        // Simple local read
        let result = GetResult {
            value: local_value,
            found: local_found,
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn put(payload: &str, state: &mut StorageState) -> Response {
        let args = PutArgs::deserialize(payload).expect("Failed to deserialize payload");
        let version = state.engine.put(args.key.clone(), args.value.clone());

        // Replicate to peers
        if state.quorum_w > 1 {
            let peers = get_peers(&state.own_addr).await;
            let needed = (state.quorum_w - 1) as usize; // local counts as 1

            let mut acks = 0;
            for peer in &peers {
                if acks >= needed {
                    break;
                }
                let result = storage::replicate_put(
                    peer,
                    args.key.clone(),
                    args.value.clone(),
                    version as i32,
                )
                .await;
                if !result.starts_with("ERROR") {
                    acks += 1;
                }
            }
            println!(
                "PUT {} replicated to {}/{} peers (W={})",
                args.key,
                acks,
                peers.len(),
                state.quorum_w
            );
        }

        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn delete(payload: &str, state: &mut StorageState) -> Response {
        let args = DeleteArgs::deserialize(payload).expect("Failed to deserialize payload");
        let version = state.engine.delete(&args.key);

        // Replicate delete to peers
        if state.quorum_w > 1 {
            let peers = get_peers(&state.own_addr).await;
            let needed = (state.quorum_w - 1) as usize;

            let mut acks = 0;
            for peer in &peers {
                if acks >= needed {
                    break;
                }
                let result =
                    storage::replicate_delete(peer, args.key.clone(), version as i32).await;
                if !result.starts_with("ERROR") {
                    acks += 1;
                }
            }
        }

        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn scan(payload: &str, state: &mut StorageState) -> Response {
        let args = ScanArgs::deserialize(payload).expect("Failed to deserialize payload");
        let entries = state.engine.scan(&args.prefix, args.limit);
        let formatted: Vec<String> = entries.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        let result = ScanResult {
            entries: formatted.join(";"),
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn replicate_put(payload: &str, state: &mut StorageState) -> Response {
        let args =
            ReplicatePutArgs::deserialize(payload).expect("Failed to deserialize payload");
        state
            .engine
            .put_versioned(args.key, args.value, args.version as u64);
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn replicate_delete(payload: &str, state: &mut StorageState) -> Response {
        let args =
            ReplicateDeleteArgs::deserialize(payload).expect("Failed to deserialize payload");
        state
            .engine
            .delete_versioned(&args.key, args.version as u64);
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn get_peers_handler(payload: &str, state: &mut StorageState) -> Response {
        let _args =
            GetPeersArgs::deserialize(payload).expect("Failed to deserialize payload");
        let peers = get_peers(&state.own_addr).await;
        let result = GetPeersResult {
            peer_count: peers.len() as i32,
            quorum_w: state.quorum_w,
            quorum_r: state.quorum_r,
        };
        Response {
            payload: result.serialize(),
        }
    }
}

async fn request_handler(request: Request, shared_state: Arc<Mutex<StorageState>>) -> Response {
    let mut state = shared_state.lock().await;
    match request.procedure_id {
        GET_PROCEDURE => handlers::get(&request.payload, &mut state).await,
        PUT_PROCEDURE => handlers::put(&request.payload, &mut state).await,
        DELETE_PROCEDURE => handlers::delete(&request.payload, &mut state).await,
        SCAN_PROCEDURE => handlers::scan(&request.payload, &mut state).await,
        REPLICATE_PUT_PROCEDURE => handlers::replicate_put(&request.payload, &mut state).await,
        REPLICATE_DELETE_PROCEDURE => {
            handlers::replicate_delete(&request.payload, &mut state).await
        }
        GET_PEERS_PROCEDURE => handlers::get_peers_handler(&request.payload, &mut state).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());

    // Per-instance data directory based on port
    let port = addr.split(':').last().unwrap_or("10600");
    let data_dir = format!("storage_data_{}", port);

    let quorum_w: i32 = std::env::var("QUORUM_W")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    let quorum_r: i32 = std::env::var("QUORUM_R")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    let state = Arc::new(Mutex::new(StorageState {
        engine: StorageEngine::new(&data_dir),
        own_addr: addr.clone(),
        quorum_w,
        quorum_r,
    }));

    // Background compaction task
    let compaction_state = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            sleep(COMPACTION_INTERVAL).await;
            println!("Running compaction check");
            compaction_state.lock().await.engine.compact();
        }
    });

    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    println!(
        "Storage service starting on {} (data_dir={}, W={}, R={})",
        addr, data_dir, quorum_w, quorum_r
    );

    server::start_server_with_state(
        &addr,
        |request, state| {
            Box::pin(request_handler(request, state))
                as Pin<Box<dyn Future<Output = Response> + Send>>
        },
        state,
    )
    .await
    .expect("Server crashed");
}
