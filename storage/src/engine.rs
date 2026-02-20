use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct VersionedValue {
    pub value: String,
    pub version: u64,
}

pub struct StorageEngine {
    data: HashMap<String, VersionedValue>,
    wal_path: PathBuf,
    snapshot_path: PathBuf,
    operations_since_snapshot: usize,
    next_version: u64,
}

const COMPACTION_THRESHOLD: usize = 1000;

impl StorageEngine {
    pub fn new(data_dir: &str) -> Self {
        fs::create_dir_all(data_dir).expect("Failed to create data directory");

        let wal_path = PathBuf::from(data_dir).join("wal.log");
        let snapshot_path = PathBuf::from(data_dir).join("snapshot.dat");

        let mut engine = StorageEngine {
            data: HashMap::new(),
            wal_path,
            snapshot_path,
            operations_since_snapshot: 0,
            next_version: 1,
        };

        engine.recover();
        engine
    }

    fn recover(&mut self) {
        // First, load snapshot if it exists
        if self.snapshot_path.exists() {
            if let Ok(file) = File::open(&self.snapshot_path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        // New format: key=value@version
                        if let Some(at_idx) = line.rfind('@') {
                            let kv = &line[..at_idx];
                            let ver_str = &line[at_idx + 1..];
                            if let Ok(version) = ver_str.parse::<u64>() {
                                let parts: Vec<&str> = kv.splitn(2, '=').collect();
                                if parts.len() == 2 {
                                    self.data.insert(
                                        parts[0].to_string(),
                                        VersionedValue {
                                            value: parts[1].to_string(),
                                            version,
                                        },
                                    );
                                    if version >= self.next_version {
                                        self.next_version = version + 1;
                                    }
                                }
                                continue;
                            }
                        }
                        // Old format: key=value (backward compatible)
                        let parts: Vec<&str> = line.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            self.data.insert(
                                parts[0].to_string(),
                                VersionedValue {
                                    value: parts[1].to_string(),
                                    version: 0,
                                },
                            );
                        }
                    }
                }
            }
        }

        // Then, replay WAL on top
        if self.wal_path.exists() {
            if let Ok(file) = File::open(&self.wal_path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        self.apply_wal_entry(&line);
                    }
                }
            }
        }

        println!(
            "Storage recovered: {} keys loaded, next_version={}",
            self.data.len(),
            self.next_version
        );
    }

    fn apply_wal_entry(&mut self, entry: &str) {
        // New versioned format: VPUT key=value@version
        if let Some(rest) = entry.strip_prefix("VPUT ") {
            if let Some(at_idx) = rest.rfind('@') {
                let kv = &rest[..at_idx];
                let ver_str = &rest[at_idx + 1..];
                if let Ok(version) = ver_str.parse::<u64>() {
                    let parts: Vec<&str> = kv.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        self.data.insert(
                            parts[0].to_string(),
                            VersionedValue {
                                value: parts[1].to_string(),
                                version,
                            },
                        );
                        if version >= self.next_version {
                            self.next_version = version + 1;
                        }
                    }
                }
            }
        } else if let Some(rest) = entry.strip_prefix("VDEL ") {
            // VDEL key@version
            if let Some(at_idx) = rest.rfind('@') {
                let key = &rest[..at_idx];
                let ver_str = &rest[at_idx + 1..];
                if let Ok(version) = ver_str.parse::<u64>() {
                    // Only delete if version is >= current
                    if let Some(current) = self.data.get(key) {
                        if version >= current.version {
                            self.data.remove(key);
                        }
                    }
                    if version >= self.next_version {
                        self.next_version = version + 1;
                    }
                }
            }
        } else if let Some(rest) = entry.strip_prefix("PUT ") {
            // Old format backward compat
            let parts: Vec<&str> = rest.splitn(2, '=').collect();
            if parts.len() == 2 {
                self.data.insert(
                    parts[0].to_string(),
                    VersionedValue {
                        value: parts[1].to_string(),
                        version: 0,
                    },
                );
            }
        } else if let Some(rest) = entry.strip_prefix("DELETE ") {
            self.data.remove(rest);
        }
    }

    fn append_wal(&mut self, entry: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.wal_path)
            .expect("Failed to open WAL");
        writeln!(file, "{}", entry).expect("Failed to write to WAL");

        self.operations_since_snapshot += 1;
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key).map(|v| &v.value)
    }

    pub fn get_versioned(&self, key: &str) -> Option<&VersionedValue> {
        self.data.get(key)
    }

    pub fn put(&mut self, key: String, value: String) -> u64 {
        let version = self.next_version;
        self.next_version += 1;
        self.append_wal(&format!("VPUT {}={}@{}", key, value, version));
        self.data.insert(
            key,
            VersionedValue {
                value,
                version,
            },
        );
        version
    }

    pub fn put_versioned(&mut self, key: String, value: String, version: u64) -> bool {
        // Only apply if version >= current
        if let Some(current) = self.data.get(&key) {
            if version < current.version {
                return false;
            }
        }
        self.append_wal(&format!("VPUT {}={}@{}", key, value, version));
        self.data.insert(
            key,
            VersionedValue {
                value,
                version,
            },
        );
        if version >= self.next_version {
            self.next_version = version + 1;
        }
        true
    }

    pub fn delete(&mut self, key: &str) -> u64 {
        let version = self.next_version;
        self.next_version += 1;
        self.append_wal(&format!("VDEL {}@{}", key, version));
        self.data.remove(key);
        version
    }

    pub fn delete_versioned(&mut self, key: &str, version: u64) -> bool {
        if let Some(current) = self.data.get(key) {
            if version < current.version {
                return false;
            }
        }
        self.append_wal(&format!("VDEL {}@{}", key, version));
        self.data.remove(key);
        if version >= self.next_version {
            self.next_version = version + 1;
        }
        true
    }

    pub fn scan(&self, prefix: &str, limit: i32) -> Vec<(String, String)> {
        let mut results: Vec<(String, String)> = self
            .data
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(k, v)| (k.clone(), v.value.clone()))
            .collect();
        results.sort_by(|a, b| a.0.cmp(&b.0));
        if limit > 0 {
            results.truncate(limit as usize);
        }
        results
    }

    pub fn compact(&mut self) {
        if self.operations_since_snapshot < COMPACTION_THRESHOLD {
            return;
        }

        println!(
            "Compacting storage: writing snapshot of {} keys",
            self.data.len()
        );

        // Write snapshot with version info
        let mut file = File::create(&self.snapshot_path).expect("Failed to create snapshot");
        for (key, vv) in &self.data {
            writeln!(file, "{}={}@{}", key, vv.value, vv.version)
                .expect("Failed to write snapshot");
        }

        // Truncate WAL
        File::create(&self.wal_path).expect("Failed to truncate WAL");
        self.operations_since_snapshot = 0;

        println!("Compaction complete");
    }
}
