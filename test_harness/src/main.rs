use rpc::{client, Request};
use std::process::{Child, Command, Stdio};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

// Service addresses
const DISCOVERY_ADDR: &str = "127.0.0.1:10200";
const ECHO_ADDR: &str = "127.0.0.1:10100";
const CONFIGURATION_ADDR: &str = "127.0.0.1:10500";
const STORAGE_ADDR: &str = "127.0.0.1:10600";
const CACHING_ADDR: &str = "127.0.0.1:10700";
const MONITORING_ADDR: &str = "127.0.0.1:10800";
const ROUTING_ADDR: &str = "127.0.0.1:10300";

async fn wait_for_service(addr: &str, timeout_secs: u64) -> bool {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);
    while tokio::time::Instant::now() < deadline {
        if TcpStream::connect(addr).await.is_ok() {
            return true;
        }
        sleep(Duration::from_millis(100)).await;
    }
    false
}

async fn send(addr: &str, procedure_id: i32, payload: String) -> String {
    let request = Request {
        procedure_id,
        payload,
    };
    match client::send_request(addr, request).await {
        Ok(response) => response.payload,
        Err(e) => format!("ERROR: {}", e),
    }
}

struct TestRunner {
    passed: u32,
    failed: u32,
}

impl TestRunner {
    fn new() -> Self {
        TestRunner {
            passed: 0,
            failed: 0,
        }
    }

    async fn run_test<F, Fut>(&mut self, name: &str, test_fn: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let result = test_fn().await;
        if result {
            println!("  [PASS] {}", name);
            self.passed += 1;
        } else {
            println!("  [FAIL] {}", name);
            self.failed += 1;
        }
    }

    fn summary(&self) {
        println!("\n========================================");
        println!(
            "Results: {} passed, {} failed, {} total",
            self.passed,
            self.failed,
            self.passed + self.failed
        );
        if self.failed == 0 {
            println!("All tests passed!");
        } else {
            println!("{} test(s) failed.", self.failed);
        }
        println!("========================================");
    }
}

fn spawn_service(name: &str, dir: &str, bin: Option<&str>) -> Child {
    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    if let Some(b) = bin {
        cmd.arg("--bin").arg(b);
    }
    cmd.current_dir(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let child = cmd.spawn().unwrap_or_else(|e| {
        panic!("Failed to spawn {}: {}", name, e);
    });
    println!("Spawned {} (pid {})", name, child.id());
    child
}

fn kill_all(children: &mut Vec<Child>) {
    for child in children.iter_mut() {
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[tokio::main]
async fn main() {
    let base_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let mut children: Vec<Child> = Vec::new();

    // Set up Ctrl-C handler
    let cleanup = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let cleanup_flag = cleanup.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        cleanup_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    println!("=== Starting Services ===\n");

    // 1. Start discovery first (needs --bin because it has extra binaries in src/bin/)
    children.push(spawn_service(
        "discovery",
        base_dir.join("discovery").to_str().unwrap(),
        Some("discovery"),
    ));
    print!("Waiting for discovery...");
    if !wait_for_service(DISCOVERY_ADDR, 15).await {
        println!(" TIMEOUT");
        kill_all(&mut children);
        std::process::exit(1);
    }
    println!(" ready");

    // 2. Start remaining services
    let services = vec![
        ("configuration", "configuration", None),
        ("storage", "storage", None),
        ("caching", "caching", None),
        ("monitoring", "monitoring", None),
        ("routing", "routing", None),
        ("echo", "echo", Some("server_v1")),
    ];

    for (name, dir, bin) in &services {
        children.push(spawn_service(
            name,
            base_dir.join(dir).to_str().unwrap(),
            *bin,
        ));
    }

    // Wait for all services to be ready
    let addrs = vec![
        ("configuration", CONFIGURATION_ADDR),
        ("storage", STORAGE_ADDR),
        ("caching", CACHING_ADDR),
        ("monitoring", MONITORING_ADDR),
        ("routing", ROUTING_ADDR),
        ("echo", ECHO_ADDR),
    ];

    for (name, addr) in &addrs {
        print!("Waiting for {}...", name);
        if !wait_for_service(addr, 30).await {
            println!(" TIMEOUT");
            kill_all(&mut children);
            std::process::exit(1);
        }
        println!(" ready");
    }

    // Give services a moment to register with discovery
    sleep(Duration::from_secs(2)).await;

    println!("\n=== Running Tests ===\n");
    let mut runner = TestRunner::new();

    // --- Discovery Tests ---
    println!("Discovery Tests:");
    {
        let services_to_query = vec![
            "echo",
            "configuration",
            "storage",
            "caching",
            "monitoring",
            "routing",
        ];
        for svc in services_to_query {
            let svc_name = svc.to_string();
            runner
                .run_test(
                    &format!("discovery query '{}'", svc),
                    || async {
                        let args = discovery::QueryArgs {
                            name: svc_name,
                        };
                        let resp = send(
                            DISCOVERY_ADDR,
                            discovery::QUERY_PROCEDURE,
                            args.serialize(),
                        )
                        .await;
                        if resp.starts_with("ERROR") {
                            println!("    {}", resp);
                            return false;
                        }
                        match discovery::QueryResult::deserialize(&resp) {
                            Ok(result) => !result.address.is_empty(),
                            Err(_) => false,
                        }
                    },
                )
                .await;
        }
    }

    // --- Configuration Tests ---
    println!("\nConfiguration Tests:");
    {
        // SET
        runner
            .run_test("config SET key", || async {
                let args = configuration::SetArgs {
                    key: "test.key".to_string(),
                    value: "test_value".to_string(),
                };
                let resp = send(
                    CONFIGURATION_ADDR,
                    configuration::SET_PROCEDURE,
                    args.serialize(),
                )
                .await;
                resp == "OK"
            })
            .await;

        // GET
        runner
            .run_test("config GET key", || async {
                let args = configuration::GetArgs {
                    key: "test.key".to_string(),
                };
                let resp = send(
                    CONFIGURATION_ADDR,
                    configuration::GET_PROCEDURE,
                    args.serialize(),
                )
                .await;
                match configuration::GetResult::deserialize(&resp) {
                    Ok(result) => result.value == "test_value",
                    Err(_) => false,
                }
            })
            .await;

        // LIST
        runner
            .run_test("config LIST prefix", || async {
                let args = configuration::ListArgs {
                    prefix: "test.".to_string(),
                };
                let resp = send(
                    CONFIGURATION_ADDR,
                    configuration::LIST_PROCEDURE,
                    args.serialize(),
                )
                .await;
                match configuration::ListResult::deserialize(&resp) {
                    Ok(result) => result.keys.contains("test.key"),
                    Err(_) => false,
                }
            })
            .await;

        // WATCH
        runner
            .run_test("config WATCH key", || async {
                let args = configuration::WatchArgs {
                    key: "test.key".to_string(),
                };
                let resp = send(
                    CONFIGURATION_ADDR,
                    configuration::WATCH_PROCEDURE,
                    args.serialize(),
                )
                .await;
                match configuration::WatchEvent::deserialize(&resp) {
                    Ok(event) => event.value == "test_value",
                    Err(_) => false,
                }
            })
            .await;

        // DELETE
        runner
            .run_test("config DELETE key", || async {
                let args = configuration::DeleteArgs {
                    key: "test.key".to_string(),
                };
                let resp = send(
                    CONFIGURATION_ADDR,
                    configuration::DELETE_PROCEDURE,
                    args.serialize(),
                )
                .await;
                resp == "OK"
            })
            .await;

        // Verify deleted
        runner
            .run_test("config GET after DELETE returns empty", || async {
                let args = configuration::GetArgs {
                    key: "test.key".to_string(),
                };
                let resp = send(
                    CONFIGURATION_ADDR,
                    configuration::GET_PROCEDURE,
                    args.serialize(),
                )
                .await;
                match configuration::GetResult::deserialize(&resp) {
                    Ok(result) => result.value.is_empty(),
                    Err(_) => false,
                }
            })
            .await;
    }

    // --- Storage Tests ---
    println!("\nStorage Tests:");
    {
        // PUT
        runner
            .run_test("storage PUT key", || async {
                let args = storage::PutArgs {
                    key: "test.item".to_string(),
                    value: "stored_value".to_string(),
                };
                let resp =
                    send(STORAGE_ADDR, storage::PUT_PROCEDURE, args.serialize()).await;
                resp == "OK"
            })
            .await;

        // GET (found)
        runner
            .run_test("storage GET key (found)", || async {
                let args = storage::GetArgs {
                    key: "test.item".to_string(),
                };
                let resp =
                    send(STORAGE_ADDR, storage::GET_PROCEDURE, args.serialize()).await;
                match storage::GetResult::deserialize(&resp) {
                    Ok(result) => result.found == 1 && result.value == "stored_value",
                    Err(_) => false,
                }
            })
            .await;

        // SCAN
        runner
            .run_test("storage SCAN prefix", || async {
                let args = storage::ScanArgs {
                    prefix: "test.".to_string(),
                    limit: 100,
                };
                let resp =
                    send(STORAGE_ADDR, storage::SCAN_PROCEDURE, args.serialize()).await;
                match storage::ScanResult::deserialize(&resp) {
                    Ok(result) => result.entries.contains("test.item=stored_value"),
                    Err(_) => false,
                }
            })
            .await;

        // DELETE
        runner
            .run_test("storage DELETE key", || async {
                let args = storage::DeleteArgs {
                    key: "test.item".to_string(),
                };
                let resp = send(STORAGE_ADDR, storage::DELETE_PROCEDURE, args.serialize())
                    .await;
                resp == "OK"
            })
            .await;

        // GET (not found)
        runner
            .run_test("storage GET after DELETE (not found)", || async {
                let args = storage::GetArgs {
                    key: "test.item".to_string(),
                };
                let resp =
                    send(STORAGE_ADDR, storage::GET_PROCEDURE, args.serialize()).await;
                match storage::GetResult::deserialize(&resp) {
                    Ok(result) => result.found == 0,
                    Err(_) => false,
                }
            })
            .await;
    }

    // --- Caching Tests ---
    println!("\nCaching Tests:");
    {
        // SET with TTL
        runner
            .run_test("cache SET with TTL", || async {
                let args = caching::SetArgs {
                    key: "cache.key".to_string(),
                    value: "cached_value".to_string(),
                    ttl_secs: 60,
                };
                let resp =
                    send(CACHING_ADDR, caching::SET_PROCEDURE, args.serialize()).await;
                resp == "OK"
            })
            .await;

        // GET (hit)
        runner
            .run_test("cache GET (hit)", || async {
                let args = caching::GetArgs {
                    key: "cache.key".to_string(),
                };
                let resp =
                    send(CACHING_ADDR, caching::GET_PROCEDURE, args.serialize()).await;
                match caching::GetResult::deserialize(&resp) {
                    Ok(result) => result.hit == 1 && result.value == "cached_value",
                    Err(_) => false,
                }
            })
            .await;

        // DELETE
        runner
            .run_test("cache DELETE", || async {
                let args = caching::DeleteArgs {
                    key: "cache.key".to_string(),
                };
                let resp = send(CACHING_ADDR, caching::DELETE_PROCEDURE, args.serialize())
                    .await;
                resp == "OK"
            })
            .await;

        // GET (miss)
        runner
            .run_test("cache GET after DELETE (miss)", || async {
                let args = caching::GetArgs {
                    key: "cache.key".to_string(),
                };
                let resp =
                    send(CACHING_ADDR, caching::GET_PROCEDURE, args.serialize()).await;
                match caching::GetResult::deserialize(&resp) {
                    Ok(result) => result.hit == 0,
                    Err(_) => false,
                }
            })
            .await;

        // STATS
        runner
            .run_test("cache STATS", || async {
                let args = caching::StatsArgs { placeholder: 0 };
                let resp = send(CACHING_ADDR, caching::STATS_PROCEDURE, args.serialize())
                    .await;
                match caching::StatsResult::deserialize(&resp) {
                    Ok(result) => {
                        // We had 1 hit and 1 miss from the tests above
                        result.hits >= 1 && result.misses >= 1
                    }
                    Err(_) => false,
                }
            })
            .await;
    }

    // --- Monitoring Tests ---
    println!("\nMonitoring Tests:");
    {
        // HEARTBEAT
        runner
            .run_test("monitoring HEARTBEAT", || async {
                let args = monitoring::HeartbeatArgs {
                    service: "test_service".to_string(),
                    status: "healthy".to_string(),
                };
                let resp = send(
                    MONITORING_ADDR,
                    monitoring::HEARTBEAT_PROCEDURE,
                    args.serialize(),
                )
                .await;
                resp == "OK"
            })
            .await;

        // REPORT metric
        runner
            .run_test("monitoring REPORT metric", || async {
                let args = monitoring::ReportArgs {
                    service: "test_service".to_string(),
                    metric: "cpu".to_string(),
                    value: 42,
                };
                let resp = send(
                    MONITORING_ADDR,
                    monitoring::REPORT_PROCEDURE,
                    args.serialize(),
                )
                .await;
                resp == "OK"
            })
            .await;

        // QUERY metric
        runner
            .run_test("monitoring QUERY metric", || async {
                let args = monitoring::QueryArgs {
                    service: "test_service".to_string(),
                    metric: "cpu".to_string(),
                };
                let resp = send(
                    MONITORING_ADDR,
                    monitoring::QUERY_PROCEDURE,
                    args.serialize(),
                )
                .await;
                match monitoring::QueryResult::deserialize(&resp) {
                    Ok(result) => result.values.contains("42"),
                    Err(_) => false,
                }
            })
            .await;

        // HEALTH
        runner
            .run_test("monitoring HEALTH check", || async {
                let args = monitoring::HealthArgs { placeholder: 0 };
                let resp = send(
                    MONITORING_ADDR,
                    monitoring::HEALTH_PROCEDURE,
                    args.serialize(),
                )
                .await;
                match monitoring::HealthResult::deserialize(&resp) {
                    Ok(result) => result.services.contains("test_service=healthy"),
                    Err(_) => false,
                }
            })
            .await;
    }

    // --- Router Tests ---
    println!("\nRouter Tests:");
    {
        // Route to configuration via router
        runner
            .run_test("router -> configuration SET", || async {
                let inner_args = configuration::SetArgs {
                    key: "routed.key".to_string(),
                    value: "routed_value".to_string(),
                };
                let route_args = routing::RouteArgs {
                    name: "configuration".to_string(),
                    procedure_id: configuration::SET_PROCEDURE,
                    payload: inner_args.serialize(),
                };
                let resp = send(
                    ROUTING_ADDR,
                    routing::ROUTE_PROCEDURE,
                    route_args.serialize(),
                )
                .await;
                resp == "OK"
            })
            .await;

        // Route to configuration GET to verify
        runner
            .run_test("router -> configuration GET", || async {
                let inner_args = configuration::GetArgs {
                    key: "routed.key".to_string(),
                };
                let route_args = routing::RouteArgs {
                    name: "configuration".to_string(),
                    procedure_id: configuration::GET_PROCEDURE,
                    payload: inner_args.serialize(),
                };
                let resp = send(
                    ROUTING_ADDR,
                    routing::ROUTE_PROCEDURE,
                    route_args.serialize(),
                )
                .await;
                match configuration::GetResult::deserialize(&resp) {
                    Ok(result) => result.value == "routed_value",
                    Err(_) => false,
                }
            })
            .await;

        // Route to storage via router
        runner
            .run_test("router -> storage PUT", || async {
                let inner_args = storage::PutArgs {
                    key: "routed.item".to_string(),
                    value: "routed_stored".to_string(),
                };
                let route_args = routing::RouteArgs {
                    name: "storage".to_string(),
                    procedure_id: storage::PUT_PROCEDURE,
                    payload: inner_args.serialize(),
                };
                let resp = send(
                    ROUTING_ADDR,
                    routing::ROUTE_PROCEDURE,
                    route_args.serialize(),
                )
                .await;
                resp == "OK"
            })
            .await;

        // Route to storage GET to verify
        runner
            .run_test("router -> storage GET", || async {
                let inner_args = storage::GetArgs {
                    key: "routed.item".to_string(),
                };
                let route_args = routing::RouteArgs {
                    name: "storage".to_string(),
                    procedure_id: storage::GET_PROCEDURE,
                    payload: inner_args.serialize(),
                };
                let resp = send(
                    ROUTING_ADDR,
                    routing::ROUTE_PROCEDURE,
                    route_args.serialize(),
                )
                .await;
                match storage::GetResult::deserialize(&resp) {
                    Ok(result) => result.found == 1 && result.value == "routed_stored",
                    Err(_) => false,
                }
            })
            .await;
    }

    // Print summary
    runner.summary();

    // Cleanup
    println!("\n=== Cleaning Up ===\n");
    kill_all(&mut children);
    println!("All services stopped.");

    // Clean up storage WAL directory
    let storage_data = base_dir.join("storage").join("storage_data");
    if storage_data.exists() {
        let _ = std::fs::remove_dir_all(&storage_data);
        println!("Cleaned up storage data directory.");
    }

    if runner.failed > 0 {
        std::process::exit(1);
    }
}
