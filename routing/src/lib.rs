use normalization::{Deserializable, NormalizationError, Serializable};
use rand::Rng;
use rpc::{ProcedureId, Response};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::Instant;

pub const ROUTE_PROCEDURE: ProcedureId = 1;
pub const ROUTE_SET_STRATEGY_PROCEDURE: ProcedureId = 2;

#[derive(Debug, Serializable, Deserializable)]
pub struct RouteArgs {
    pub name: String,
    pub procedure_id: i32,
    pub payload: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct RouteResult {
    pub payload: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct SetStrategyArgs {
    pub strategy: String,
}

struct BackendPool {
    address: String,
    connections: VecDeque<TcpStream>,
}

pub struct ConnectionPool {
    system_name: String,
    backends: Vec<BackendPool>,
    strategy: String,
    next_index: usize,
    max_per_backend: usize,
    last_refresh: Instant,
}

const REFRESH_INTERVAL_SECS: u64 = 10;

impl ConnectionPool {
    pub fn new(system_name: String, max_size: usize) -> Self {
        Self {
            system_name,
            backends: Vec::new(),
            strategy: "round-robin".to_string(),
            next_index: 0,
            max_per_backend: max_size,
            last_refresh: Instant::now() - tokio::time::Duration::from_secs(REFRESH_INTERVAL_SECS + 1),
        }
    }

    pub fn set_strategy(&mut self, strategy: String) {
        self.strategy = strategy;
    }

    async fn refresh_backends(&mut self) {
        if self.last_refresh.elapsed().as_secs() < REFRESH_INTERVAL_SECS {
            return;
        }
        self.last_refresh = Instant::now();

        let result = discovery::list(self.system_name.clone()).await;
        let addresses: Vec<String> = result
            .addresses
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        // Add new backends
        for addr in &addresses {
            if !self.backends.iter().any(|b| &b.address == addr) {
                println!("Routing: adding backend {} for {}", addr, self.system_name);
                self.backends.push(BackendPool {
                    address: addr.clone(),
                    connections: VecDeque::new(),
                });
            }
        }

        // Remove stale backends
        self.backends.retain(|b| addresses.contains(&b.address));
    }

    fn select_backend(&mut self) -> Option<usize> {
        if self.backends.is_empty() {
            return None;
        }

        match self.strategy.as_str() {
            "least-connections" => {
                let mut best = 0;
                for i in 1..self.backends.len() {
                    if self.backends[i].connections.len() < self.backends[best].connections.len() {
                        best = i;
                    }
                }
                Some(best)
            }
            "random" => {
                let idx = rand::thread_rng().gen_range(0..self.backends.len());
                Some(idx)
            }
            "pick-2" => {
                if self.backends.len() == 1 {
                    return Some(0);
                }
                let mut rng = rand::thread_rng();
                let a = rng.gen_range(0..self.backends.len());
                let mut b = a;
                while b == a {
                    b = rng.gen_range(0..self.backends.len());
                }
                if self.backends[a].connections.len() <= self.backends[b].connections.len() {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            // round-robin default
            _ => {
                let idx = self.next_index % self.backends.len();
                self.next_index = (self.next_index + 1) % self.backends.len();
                Some(idx)
            }
        }
    }

    pub async fn get(&mut self) -> Option<(TcpStream, String)> {
        self.refresh_backends().await;

        let backend_idx = self.select_backend()?;
        let backend = &mut self.backends[backend_idx];
        let address = backend.address.clone();

        // Try to reuse a pooled connection
        if let Some(conn) = backend.connections.pop_front() {
            return Some((conn, address));
        }

        // Open a new connection
        match TcpStream::connect(&address).await {
            Ok(conn) => Some((conn, address)),
            Err(e) => {
                eprintln!("Failed to connect to {}: {}", address, e);
                None
            }
        }
    }

    pub fn release(&mut self, conn: TcpStream, address: String) {
        if let Some(backend) = self.backends.iter_mut().find(|b| b.address == address) {
            if backend.connections.len() < self.max_per_backend {
                backend.connections.push_back(conn);
            }
        }
        println!("Pool for {}: {} backends", self.system_name, self.backends.len());
    }

    pub async fn send_request(
        &mut self,
        procedure_id: ProcedureId,
        payload: &str,
    ) -> Result<String, String> {
        if let Some((mut socket, address)) = self.get().await {
            let serialized = format!("{}:{}\n", procedure_id, payload);
            println!("Sending to {}: {}", address, serialized);
            if let Err(e) = socket.write_all(serialized.as_bytes()).await {
                eprintln!("Failed to forward request: {}", e);
                return Err(format!("Failed to send request: {}", e));
            }

            let mut buffer = vec![0u8; 1024];
            let n = socket
                .read(&mut buffer)
                .await
                .expect("Failed to read from socket");
            let response_data = String::from_utf8_lossy(&buffer[..n]).to_string();
            println!("Received: {}", response_data);

            self.release(socket, address);
            Ok(response_data)
        } else {
            Err("Service not available".to_string())
        }
    }
}

pub struct Router {
    pools: Arc<Mutex<HashMap<String, ConnectionPool>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn send_request(
        &self,
        name: String,
        procedure_id: ProcedureId,
        payload: &str,
    ) -> Response {
        let mut pools = self.pools.lock().await;
        let pool = pools
            .entry(name.clone())
            .or_insert_with(|| ConnectionPool::new(name.clone(), 10));

        route_request(
            RouteArgs {
                name,
                procedure_id,
                payload: payload.to_string(),
            },
            pool,
        )
        .await
    }
}

async fn route_request(args: RouteArgs, pool: &mut ConnectionPool) -> Response {
    match pool.send_request(args.procedure_id, &args.payload).await {
        Ok(response_data) => Response {
            payload: response_data,
        },
        Err(err_msg) => Response { payload: err_msg },
    }
}
