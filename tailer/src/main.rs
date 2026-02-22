use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::{server, Response, ProcedureId};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};
use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

const SYSTEM_NAME: &str = "tailer";
const SYSTEM_ADDRESS: &str = "127.0.0.1:10400";
const TAIL_INTERVAL: Duration = Duration::from_secs(2);
const PEER_REFRESH_INTERVAL: Duration = Duration::from_secs(10);
const STATS_PROCEDURE: ProcedureId = 1;

#[allow(dead_code)]
#[derive(Debug, Serializable, Deserializable)]
struct StatsArgs {
    placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
struct StatsResult {
    entries_tailed: i32,
    entries_replicated: i32,
    errors: i32,
    lag_bytes: i32,
}

enum WalEntry {
    Put { key: String, value: String, version: u64 },
    Delete { key: String, version: u64 },
}

struct WalTailer {
    data_dir: String,
    wal_offset: u64,
}

struct TailerState {
    entries_tailed: i32,
    entries_replicated: i32,
    errors: i32,
    lag_bytes: i32,
}

impl WalTailer {
    fn new(data_dir: String) -> Self {
        // Start at end of current WAL (don't replay history on first start)
        let wal_path = format!("{}/wal.log", data_dir);
        let offset = fs::metadata(&wal_path)
            .map(|m| m.len())
            .unwrap_or(0);
        WalTailer { data_dir, wal_offset: offset }
    }

    fn read_new_entries(&mut self) -> Vec<WalEntry> {
        let wal_path = format!("{}/wal.log", self.data_dir);
        let file_size = match fs::metadata(&wal_path) {
            Ok(m) => m.len(),
            Err(_) => return Vec::new(),
        };

        // Compaction detected: WAL was truncated
        if file_size < self.wal_offset {
            println!("[tailer] WAL compaction detected in {}, re-reading", self.data_dir);
            // After compaction, snapshot has latest state and WAL is empty/small
            // Read snapshot entries first
            let mut entries = Vec::new();
            let snap_path = format!("{}/snapshot.dat", self.data_dir);
            if let Ok(file) = fs::File::open(&snap_path) {
                let reader = BufReader::new(file);
                for line in reader.lines().flatten() {
                    if let Some(entry) = Self::parse_snapshot_line(&line) {
                        entries.push(entry);
                    }
                }
            }
            // Then read full WAL
            if let Ok(file) = fs::File::open(&wal_path) {
                let reader = BufReader::new(file);
                for line in reader.lines().flatten() {
                    if let Some(entry) = Self::parse_wal_line(&line) {
                        entries.push(entry);
                    }
                }
            }
            self.wal_offset = file_size;
            return entries;
        }

        if file_size == self.wal_offset {
            return Vec::new();
        }

        // Read new WAL entries from offset
        let mut entries = Vec::new();
        if let Ok(file) = fs::File::open(&wal_path) {
            let mut reader = BufReader::new(file);
            if reader.seek(SeekFrom::Start(self.wal_offset)).is_ok() {
                for line in reader.lines().flatten() {
                    if let Some(entry) = Self::parse_wal_line(&line) {
                        entries.push(entry);
                    }
                }
            }
        }
        self.wal_offset = file_size;
        entries
    }

    fn parse_wal_line(line: &str) -> Option<WalEntry> {
        if let Some(rest) = line.strip_prefix("VPUT ") {
            if let Some(at_idx) = rest.rfind('@') {
                let kv = &rest[..at_idx];
                let ver_str = &rest[at_idx + 1..];
                if let Ok(version) = ver_str.parse::<u64>() {
                    let parts: Vec<&str> = kv.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        return Some(WalEntry::Put {
                            key: parts[0].to_string(),
                            value: parts[1].to_string(),
                            version,
                        });
                    }
                }
            }
        } else if let Some(rest) = line.strip_prefix("VDEL ") {
            if let Some(at_idx) = rest.rfind('@') {
                let key = rest[..at_idx].to_string();
                let ver_str = &rest[at_idx + 1..];
                if let Ok(version) = ver_str.parse::<u64>() {
                    return Some(WalEntry::Delete { key, version });
                }
            }
        }
        None
    }

    fn parse_snapshot_line(line: &str) -> Option<WalEntry> {
        // Snapshot format: key=value@version
        if let Some(at_idx) = line.rfind('@') {
            let kv = &line[..at_idx];
            let ver_str = &line[at_idx + 1..];
            if let Ok(version) = ver_str.parse::<u64>() {
                let parts: Vec<&str> = kv.splitn(2, '=').collect();
                if parts.len() == 2 {
                    return Some(WalEntry::Put {
                        key: parts[0].to_string(),
                        value: parts[1].to_string(),
                        version,
                    });
                }
            }
        }
        None
    }
}

#[tokio::main]
async fn main() {
    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());

    let state = Arc::new(Mutex::new(TailerState {
        entries_tailed: 0,
        entries_replicated: 0,
        errors: 0,
        lag_bytes: 0,
    }));

    discovery::register(SYSTEM_NAME.to_string(), addr.clone());
    println!("Tailer service starting on {}", addr);

    // Tailing loop
    let tail_state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut tailers: Vec<WalTailer> = Vec::new();
        let mut remote_peers: Vec<String> = Vec::new();
        let mut last_peer_refresh = Instant::now() - PEER_REFRESH_INTERVAL;

        loop {
            // Refresh peers periodically
            if last_peer_refresh.elapsed() >= PEER_REFRESH_INTERVAL {
                // Get local storage instances
                let local = discovery::list_local("storage".to_string()).await;
                let local_addrs: Vec<String> = local.addresses
                    .split(';')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();

                // Get all storage instances
                let all = discovery::list("storage".to_string()).await;
                let all_addrs: Vec<String> = all.addresses
                    .split(';')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();

                // Remote = all minus local
                remote_peers = all_addrs
                    .into_iter()
                    .filter(|a| !local_addrs.contains(a))
                    .collect();

                // Create tailers for any new local instances
                for addr in &local_addrs {
                    let port = addr.split(':').last().unwrap_or("10600");
                    let data_dir = format!("storage_data_{}", port);
                    if !tailers.iter().any(|t| t.data_dir == data_dir) {
                        println!("[tailer] Watching {}", data_dir);
                        tailers.push(WalTailer::new(data_dir));
                    }
                }

                last_peer_refresh = Instant::now();
                println!("[tailer] {} local storage dirs, {} remote peers", tailers.len(), remote_peers.len());
            }

            // Tail each local storage WAL
            let mut total_entries = 0;
            let mut total_replicated = 0;
            let mut total_errors = 0;

            for tailer in &mut tailers {
                let entries = tailer.read_new_entries();
                total_entries += entries.len();

                for entry in &entries {
                    for peer in &remote_peers {
                        let result = match entry {
                            WalEntry::Put { key, value, version } => {
                                storage::replicate_put(peer, key.clone(), value.clone(), *version as i32).await
                            }
                            WalEntry::Delete { key, version } => {
                                storage::replicate_delete(peer, key.clone(), *version as i32).await
                            }
                        };
                        if result.starts_with("ERROR") {
                            total_errors += 1;
                        } else {
                            total_replicated += 1;
                        }
                    }
                }
            }

            if total_entries > 0 {
                println!("[tailer] Tailed {} entries, replicated {}, errors {}",
                    total_entries, total_replicated, total_errors);
            }

            // Update stats
            {
                let mut s = tail_state.lock().await;
                s.entries_tailed += total_entries as i32;
                s.entries_replicated += total_replicated as i32;
                s.errors += total_errors as i32;
            }

            sleep(TAIL_INTERVAL).await;
        }
    });

    // RPC server for stats
    server::start_server_with_state(
        &addr,
        |request, state| {
            Box::pin(async move {
                let s = state.lock().await;
                match request.procedure_id {
                    STATS_PROCEDURE => {
                        let result = StatsResult {
                            entries_tailed: s.entries_tailed,
                            entries_replicated: s.entries_replicated,
                            errors: s.errors,
                            lag_bytes: s.lag_bytes,
                        };
                        Response {
                            payload: result.serialize(),
                        }
                    }
                    _ => Response {
                        payload: "Unknown procedure".to_string(),
                    },
                }
            }) as Pin<Box<dyn Future<Output = Response> + Send>>
        },
        state,
    )
    .await
    .expect("Server crashed");
}
