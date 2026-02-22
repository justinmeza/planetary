use rand::Rng;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

const LISTEN_ADDR: &str = "0.0.0.0:8080";
const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(3);
const BACKEND_REFRESH_INTERVAL: Duration = Duration::from_secs(5);
const CLEANUP_INTERVAL: Duration = Duration::from_secs(60);
const READ_TIMEOUT: Duration = Duration::from_secs(5);

const RATE_CAPACITY: f64 = 30.0;
const RATE_REFILL: f64 = 2.0;
const BLACKHOLE_THRESHOLD: u32 = 10;
const BLACKHOLE_WINDOW: Duration = Duration::from_secs(60);
const BLACKHOLE_DURATION: Duration = Duration::from_secs(300);

const SHED_THRESHOLD: f64 = 0.8;
const MAX_CONNECTIONS_PER_BACKEND: usize = 100;
const REGIONS: &[(&str, &str)] = &[("sfo", "10.0.0.1"), ("nyc", "10.0.0.2"), ("ams", "10.0.0.3")];

struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new() -> Self {
        TokenBucket {
            tokens: RATE_CAPACITY,
            last_refill: Instant::now(),
        }
    }

    fn try_consume(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * RATE_REFILL).min(RATE_CAPACITY);
        self.last_refill = now;
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

struct Backend {
    address: String,
    healthy: bool,
    active_connections: usize,
    local: bool,
}

struct LoadBalancer {
    backends: Vec<Backend>,
    strategy: String,
    next_index: usize,
    rate_limits: HashMap<IpAddr, TokenBucket>,
    blacklist: HashMap<IpAddr, Instant>,
    violations: HashMap<IpAddr, (u32, Instant)>,
    drained_regions: Vec<String>,
    own_region: String,
}

impl LoadBalancer {
    fn new(strategy: String, own_region: String) -> Self {
        LoadBalancer {
            backends: Vec::new(),
            strategy,
            next_index: 0,
            rate_limits: HashMap::new(),
            blacklist: HashMap::new(),
            violations: HashMap::new(),
            drained_regions: Vec::new(),
            own_region,
        }
    }

    fn is_blackholed(&self, ip: &IpAddr) -> bool {
        if let Some(expiry) = self.blacklist.get(ip) {
            Instant::now() < *expiry
        } else {
            false
        }
    }

    fn check_rate_limit(&mut self, ip: IpAddr) -> bool {
        let bucket = self
            .rate_limits
            .entry(ip)
            .or_insert_with(TokenBucket::new);
        bucket.try_consume()
    }

    fn record_violation(&mut self, ip: IpAddr) {
        let now = Instant::now();
        let (count, window_start) = self
            .violations
            .entry(ip)
            .or_insert((0, now));
        if now.duration_since(*window_start) > BLACKHOLE_WINDOW {
            *count = 0;
            *window_start = now;
        }
        *count += 1;
        if *count >= BLACKHOLE_THRESHOLD {
            let expiry = now + BLACKHOLE_DURATION;
            self.blacklist.insert(ip, expiry);
            self.violations.remove(&ip);
            println!("Blackholed {} for {}s", ip, BLACKHOLE_DURATION.as_secs());
        }
    }

    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.blacklist.retain(|_, expiry| now < *expiry);
        self.violations
            .retain(|_, (_, window_start)| now.duration_since(*window_start) <= BLACKHOLE_WINDOW);
        // Remove rate limit buckets that haven't been used in a while (full buckets from long ago)
        self.rate_limits.retain(|_, bucket| {
            now.duration_since(bucket.last_refill) < BLACKHOLE_DURATION
        });
    }

    fn select_from(&mut self, candidates: &[usize]) -> Option<usize> {
        if candidates.is_empty() {
            return None;
        }

        match self.strategy.as_str() {
            "least-connections" => {
                let mut best = candidates[0];
                for &i in candidates {
                    if self.backends[i].active_connections
                        < self.backends[best].active_connections
                    {
                        best = i;
                    }
                }
                Some(best)
            }
            "random" => {
                let idx = rand::thread_rng().gen_range(0..candidates.len());
                Some(candidates[idx])
            }
            "pick-2" => {
                if candidates.len() == 1 {
                    return Some(candidates[0]);
                }
                let mut rng = rand::thread_rng();
                let a = candidates[rng.gen_range(0..candidates.len())];
                let mut b = a;
                while b == a {
                    b = candidates[rng.gen_range(0..candidates.len())];
                }
                if self.backends[a].active_connections <= self.backends[b].active_connections {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            // "round-robin" is the default
            _ => {
                let start = self.next_index;
                let total = self.backends.len();
                // Find the first candidate at or after next_index
                for offset in 0..total {
                    let idx = (start + offset) % total;
                    if candidates.contains(&idx) {
                        self.next_index = (idx + 1) % total;
                        return Some(idx);
                    }
                }
                None
            }
        }
    }

    fn local_utilization(&self) -> f64 {
        let local_healthy: Vec<usize> = self
            .backends
            .iter()
            .enumerate()
            .filter(|(_, b)| b.healthy && b.local)
            .map(|(i, _)| i)
            .collect();
        if local_healthy.is_empty() {
            return 0.0;
        }
        let total_conns: usize = local_healthy.iter().map(|&i| self.backends[i].active_connections).sum();
        total_conns as f64 / (local_healthy.len() * MAX_CONNECTIONS_PER_BACKEND) as f64
    }

    fn is_shedding(&self) -> bool {
        let local_healthy: Vec<usize> = self
            .backends
            .iter()
            .enumerate()
            .filter(|(_, b)| b.healthy && b.local)
            .map(|(i, _)| i)
            .collect();
        if local_healthy.is_empty() {
            return true;
        }
        self.local_utilization() >= SHED_THRESHOLD
    }

    fn select_backend(&mut self) -> Option<usize> {
        // Filter out backends in drained regions
        let eligible: Vec<usize> = self
            .backends
            .iter()
            .enumerate()
            .filter(|(_, b)| {
                if !b.healthy {
                    return false;
                }
                let region = region_for_address(&b.address, &self.own_region);
                !self.drained_regions.contains(&region)
            })
            .map(|(i, _)| i)
            .collect();

        if eligible.is_empty() {
            return None;
        }

        let local_healthy: Vec<usize> = eligible.iter().copied().filter(|&i| self.backends[i].local).collect();
        let remote_healthy: Vec<usize> = eligible.iter().copied().filter(|&i| !self.backends[i].local).collect();

        // Calculate local utilization
        let local_util = if local_healthy.is_empty() {
            1.0 // force shedding if no local backends
        } else {
            let total_conns: usize = local_healthy.iter().map(|&i| self.backends[i].active_connections).sum();
            total_conns as f64 / (local_healthy.len() * MAX_CONNECTIONS_PER_BACKEND) as f64
        };

        if local_util < SHED_THRESHOLD && !local_healthy.is_empty() {
            // Route to local only
            self.select_from(&local_healthy)
        } else {
            // Shedding: route to all eligible backends
            let mut all = local_healthy;
            all.extend(remote_healthy);
            self.select_from(&all)
        }
    }

    fn status_json(&self) -> String {
        let mut entries = Vec::new();
        for b in &self.backends {
            entries.push(format!(
                "{{\"address\":\"{}\",\"healthy\":{},\"active_connections\":{},\"local\":{}}}",
                b.address, b.healthy, b.active_connections, b.local
            ));
        }
        let drained: Vec<String> = self.drained_regions.iter().map(|r| format!("\"{}\"", r)).collect();
        format!(
            "{{\"strategy\":\"{}\",\"backend_count\":{},\"shedding\":{},\"local_utilization\":{:.4},\"drained_regions\":[{}],\"own_region\":\"{}\",\"shed_threshold\":{},\"backends\":[{}]}}",
            self.strategy,
            self.backends.len(),
            self.is_shedding(),
            self.local_utilization(),
            drained.join(","),
            self.own_region,
            SHED_THRESHOLD,
            entries.join(",")
        )
    }

    fn refresh_backends(&mut self, addresses: &[String]) {
        // Add new backends
        for addr in addresses {
            if !self.backends.iter().any(|b| &b.address == addr) {
                println!("Adding backend: {}", addr);
                self.backends.push(Backend {
                    address: addr.clone(),
                    healthy: true,
                    active_connections: 0,
                    local: is_local(addr),
                });
            }
        }

        // Remove stale backends (not in discovery anymore)
        self.backends.retain(|b| addresses.contains(&b.address));
    }
}

fn is_local(addr: &str) -> bool {
    addr.starts_with("127.0.0.1")
}

fn region_for_address(addr: &str, own_region: &str) -> String {
    if addr.starts_with("127.0.0.1") {
        return own_region.to_string();
    }
    for &(region, ip) in REGIONS {
        if addr.starts_with(ip) {
            return region.to_string();
        }
    }
    "unknown".to_string()
}

fn report_metric(metric: &str, value: i32) {
    let args = monitoring::ReportArgs {
        service: "loadbalancer".to_string(),
        metric: metric.to_string(),
        value,
    };
    let payload = args.serialize();
    tokio::spawn(async move {
        let request = rpc::Request {
            procedure_id: monitoring::REPORT_PROCEDURE,
            payload,
        };
        let _ = rpc::client::send_request(monitoring::SYSTEM_ADDRESS, request).await;
    });
}

#[tokio::main]
async fn main() {
    let addr = std::env::var("PORT")
        .map(|p| format!("0.0.0.0:{}", p))
        .unwrap_or_else(|_| LISTEN_ADDR.to_string());

    let strategy = std::env::var("STRATEGY").unwrap_or_else(|_| "round-robin".to_string());
    let own_region = std::env::var("REGION").unwrap_or_else(|_| "sfo".to_string());
    let lb = Arc::new(Mutex::new(LoadBalancer::new(strategy, own_region)));

    // Background: refresh backends from discovery
    let refresh_lb = Arc::clone(&lb);
    tokio::spawn(async move {
        loop {
            sleep(BACKEND_REFRESH_INTERVAL).await;
            let result = discovery::list("frontend".to_string()).await;
            let addresses: Vec<String> = result
                .addresses
                .split(';')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            if !addresses.is_empty() {
                refresh_lb.lock().await.refresh_backends(&addresses);
            }
        }
    });

    // Background: health check backends
    let health_lb = Arc::clone(&lb);
    tokio::spawn(async move {
        loop {
            sleep(HEALTH_CHECK_INTERVAL).await;
            let mut lb = health_lb.lock().await;
            for backend in lb.backends.iter_mut() {
                let was_healthy = backend.healthy;
                backend.healthy = TcpStream::connect(&backend.address).await.is_ok();
                if was_healthy != backend.healthy {
                    println!(
                        "Backend {} is now {}",
                        backend.address,
                        if backend.healthy { "healthy" } else { "unhealthy" }
                    );
                }
            }
        }
    });

    // Background: cleanup expired rate limits and blacklist entries
    let cleanup_lb = Arc::clone(&lb);
    tokio::spawn(async move {
        loop {
            sleep(CLEANUP_INTERVAL).await;
            cleanup_lb.lock().await.cleanup_expired();
        }
    });

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind load balancer");
    println!("Load balancer running on http://{}", addr);

    loop {
        let (mut client, client_addr) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Accept error: {}", e);
                continue;
            }
        };

        let lb = Arc::clone(&lb);
        tokio::spawn(async move {
            let client_ip = client_addr.ip();

            // Blackhole check — before reading request
            {
                let lb_guard = lb.lock().await;
                if lb_guard.is_blackholed(&client_ip) {
                    drop(lb_guard);
                    report_metric("blackholed", 1);
                    let response = b"HTTP/1.1 429 Too Many Requests\r\nRetry-After: 300\r\nContent-Length: 20\r\nConnection: close\r\n\r\n429 Too Many Requests";
                    let _ = client.write_all(response).await;
                    return;
                }
            }

            // Read request with timeout (slowloris protection)
            let mut request_buf = vec![0u8; 65536];
            let n = match tokio::time::timeout(READ_TIMEOUT, client.read(&mut request_buf)).await {
                Ok(Ok(0)) => return,
                Ok(Ok(n)) => n,
                Ok(Err(_)) => return,
                Err(_) => {
                    // Timeout — close silently
                    return;
                }
            };

            // Rate limit check
            {
                let mut lb_guard = lb.lock().await;
                if !lb_guard.check_rate_limit(client_ip) {
                    lb_guard.record_violation(client_ip);
                    drop(lb_guard);
                    report_metric("rate_limited", 1);
                    let response = b"HTTP/1.1 429 Too Many Requests\r\nRetry-After: 1\r\nContent-Length: 20\r\nConnection: close\r\n\r\n429 Too Many Requests";
                    let _ = client.write_all(response).await;
                    return;
                }
            }

            let request_bytes = &request_buf[..n];

            // Check for introspection endpoints
            let request_str = String::from_utf8_lossy(request_bytes);
            let first_line = request_str.lines().next().unwrap_or("");

            if first_line.starts_with("GET /__lb_status") {
                if !client_ip.is_loopback() {
                    let response = b"HTTP/1.1 403 Forbidden\r\nContent-Length: 13\r\nConnection: close\r\n\r\n403 Forbidden";
                    let _ = client.write_all(response).await;
                    return;
                }
                let status = lb.lock().await.status_json();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status.len(),
                    status
                );
                let _ = client.write_all(response.as_bytes()).await;
                return;
            }

            if first_line.starts_with("POST /__lb_strategy") {
                if !client_ip.is_loopback() {
                    let response = b"HTTP/1.1 403 Forbidden\r\nContent-Length: 13\r\nConnection: close\r\n\r\n403 Forbidden";
                    let _ = client.write_all(response).await;
                    return;
                }

                // Parse body for strategy
                let body = if let Some(idx) = request_str.find("\r\n\r\n") {
                    request_str[idx + 4..].trim().to_string()
                } else {
                    String::new()
                };

                // Extract strategy from form: strategy=round-robin
                let new_strategy = body
                    .split('&')
                    .find_map(|pair| {
                        let parts: Vec<&str> = pair.splitn(2, '=').collect();
                        if parts.len() == 2 && parts[0] == "strategy" {
                            Some(parts[1].to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();

                if !new_strategy.is_empty() {
                    lb.lock().await.strategy = new_strategy.clone();
                    println!("Strategy changed to: {}", new_strategy);
                }

                let resp_body = format!("{{\"strategy\":\"{}\"}}", new_strategy);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    resp_body.len(),
                    resp_body
                );
                let _ = client.write_all(response.as_bytes()).await;
                return;
            }

            if first_line.starts_with("POST /__lb_drain") {
                if !client_ip.is_loopback() {
                    let response = b"HTTP/1.1 403 Forbidden\r\nContent-Length: 13\r\nConnection: close\r\n\r\n403 Forbidden";
                    let _ = client.write_all(response).await;
                    return;
                }

                let body = if let Some(idx) = request_str.find("\r\n\r\n") {
                    request_str[idx + 4..].trim().to_string()
                } else {
                    String::new()
                };

                let region = body
                    .split('&')
                    .find_map(|pair| {
                        let parts: Vec<&str> = pair.splitn(2, '=').collect();
                        if parts.len() == 2 && parts[0] == "region" {
                            Some(parts[1].to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();

                if !region.is_empty() {
                    let mut lb_guard = lb.lock().await;
                    if !lb_guard.drained_regions.contains(&region) {
                        lb_guard.drained_regions.push(region.clone());
                        println!("Drained region: {}", region);
                    }
                    let drained: Vec<String> = lb_guard.drained_regions.iter().map(|r| format!("\"{}\"", r)).collect();
                    let resp_body = format!("{{\"drained\":[{}]}}", drained.join(","));
                    drop(lb_guard);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        resp_body.len(),
                        resp_body
                    );
                    let _ = client.write_all(response.as_bytes()).await;
                } else {
                    let response = b"HTTP/1.1 400 Bad Request\r\nContent-Length: 15\r\nConnection: close\r\n\r\n400 Bad Request";
                    let _ = client.write_all(response).await;
                }
                return;
            }

            if first_line.starts_with("POST /__lb_undrain") {
                if !client_ip.is_loopback() {
                    let response = b"HTTP/1.1 403 Forbidden\r\nContent-Length: 13\r\nConnection: close\r\n\r\n403 Forbidden";
                    let _ = client.write_all(response).await;
                    return;
                }

                let body = if let Some(idx) = request_str.find("\r\n\r\n") {
                    request_str[idx + 4..].trim().to_string()
                } else {
                    String::new()
                };

                let region = body
                    .split('&')
                    .find_map(|pair| {
                        let parts: Vec<&str> = pair.splitn(2, '=').collect();
                        if parts.len() == 2 && parts[0] == "region" {
                            Some(parts[1].to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();

                if !region.is_empty() {
                    let mut lb_guard = lb.lock().await;
                    lb_guard.drained_regions.retain(|r| r != &region);
                    println!("Undrained region: {}", region);
                    let drained: Vec<String> = lb_guard.drained_regions.iter().map(|r| format!("\"{}\"", r)).collect();
                    let resp_body = format!("{{\"drained\":[{}]}}", drained.join(","));
                    drop(lb_guard);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        resp_body.len(),
                        resp_body
                    );
                    let _ = client.write_all(response.as_bytes()).await;
                } else {
                    let response = b"HTTP/1.1 400 Bad Request\r\nContent-Length: 15\r\nConnection: close\r\n\r\n400 Bad Request";
                    let _ = client.write_all(response).await;
                }
                return;
            }

            // Select a backend
            let (backend_addr, backend_idx) = {
                let mut lb = lb.lock().await;
                match lb.select_backend() {
                    Some(idx) => {
                        lb.backends[idx].active_connections += 1;
                        (lb.backends[idx].address.clone(), idx)
                    }
                    None => {
                        let response = b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 19\r\nConnection: close\r\n\r\n503 No Backends Up";
                        let _ = client.write_all(response).await;
                        return;
                    }
                }
            };

            println!(
                "Proxying request from {} to {}",
                client_addr, backend_addr
            );

            // Connect to the backend
            let result = async {
                let mut backend = match TcpStream::connect(&backend_addr).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Failed to connect to backend {}: {}", backend_addr, e);
                        let response = b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 15\r\nConnection: close\r\n\r\n502 Bad Gateway";
                        let _ = client.write_all(response).await;
                        return;
                    }
                };

                if backend.write_all(request_bytes).await.is_err() {
                    let response = b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 15\r\nConnection: close\r\n\r\n502 Bad Gateway";
                    let _ = client.write_all(response).await;
                    return;
                }

                let mut response_buf = vec![0u8; 131072];
                let mut total_read = 0;

                loop {
                    match backend.read(&mut response_buf[total_read..]).await {
                        Ok(0) => break,
                        Ok(n) => {
                            total_read += n;
                            if total_read >= response_buf.len() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }

                if total_read > 0 {
                    let _ = client.write_all(&response_buf[..total_read]).await;
                }
            }
            .await;

            // Decrement active connections
            {
                let mut lb = lb.lock().await;
                if backend_idx < lb.backends.len() {
                    lb.backends[backend_idx].active_connections =
                        lb.backends[backend_idx].active_connections.saturating_sub(1);
                }
            }

            result
        });
    }
}
