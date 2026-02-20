use monitoring::{
    HeartbeatArgs, HealthArgs, HealthResult, QueryArgs, QueryResult, ReportArgs,
    HEARTBEAT_PROCEDURE, HEALTH_PROCEDURE, QUERY_PROCEDURE, REPORT_PROCEDURE, SYSTEM_ADDRESS,
    SYSTEM_NAME,
};
use rpc::{server, Request, Response};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);
const CLEANUP_INTERVAL: Duration = Duration::from_secs(10);
const MAX_METRIC_WINDOW: usize = 100;

struct ServiceHealth {
    status: String,
    last_heartbeat: Instant,
}

struct MonitoringState {
    health: HashMap<String, ServiceHealth>,
    metrics: HashMap<String, Vec<i32>>,
}

impl MonitoringState {
    fn new() -> Self {
        MonitoringState {
            health: HashMap::new(),
            metrics: HashMap::new(),
        }
    }

    fn check_stale_services(&mut self) {
        let now = Instant::now();
        for (service, health) in self.health.iter_mut() {
            if now.duration_since(health.last_heartbeat) > HEARTBEAT_TIMEOUT {
                if health.status != "unhealthy" {
                    println!("Service {} marked unhealthy (heartbeat timeout)", service);
                    health.status = "unhealthy".to_string();
                }
            }
        }
    }
}

fn metric_key(service: &str, metric: &str) -> String {
    format!("{}:{}", service, metric)
}

mod handlers {
    use super::*;

    pub async fn report(payload: &str, state: &mut MonitoringState) -> Response {
        let args = ReportArgs::deserialize(payload).expect("Failed to deserialize payload");
        let key = metric_key(&args.service, &args.metric);
        let values = state.metrics.entry(key).or_insert_with(Vec::new);
        values.push(args.value);
        // Keep only the last N values (rolling window)
        if values.len() > MAX_METRIC_WINDOW {
            values.remove(0);
        }
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn heartbeat(payload: &str, state: &mut MonitoringState) -> Response {
        let args = HeartbeatArgs::deserialize(payload).expect("Failed to deserialize payload");
        let health = state.health.entry(args.service.clone()).or_insert(ServiceHealth {
            status: args.status.clone(),
            last_heartbeat: Instant::now(),
        });
        health.status = args.status;
        health.last_heartbeat = Instant::now();
        println!("Heartbeat from {}", args.service);
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn query(payload: &str, state: &mut MonitoringState) -> Response {
        let args = QueryArgs::deserialize(payload).expect("Failed to deserialize payload");
        let key = metric_key(&args.service, &args.metric);
        let values = state.metrics.get(&key);
        let result = match values {
            Some(vals) => {
                let formatted: Vec<String> = vals.iter().map(|v| v.to_string()).collect();
                QueryResult {
                    values: formatted.join(","),
                }
            }
            None => QueryResult {
                values: String::new(),
            },
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn health(payload: &str, state: &mut MonitoringState) -> Response {
        let _args = HealthArgs::deserialize(payload).expect("Failed to deserialize payload");
        let services: Vec<String> = state
            .health
            .iter()
            .map(|(name, h)| format!("{}={}", name, h.status))
            .collect();
        let result = HealthResult {
            services: services.join(";"),
        };
        Response {
            payload: result.serialize(),
        }
    }
}

async fn request_handler(request: Request, shared_state: Arc<Mutex<MonitoringState>>) -> Response {
    let mut state = shared_state.lock().await;
    match request.procedure_id {
        REPORT_PROCEDURE => handlers::report(&request.payload, &mut state).await,
        HEARTBEAT_PROCEDURE => handlers::heartbeat(&request.payload, &mut state).await,
        QUERY_PROCEDURE => handlers::query(&request.payload, &mut state).await,
        HEALTH_PROCEDURE => handlers::health(&request.payload, &mut state).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(MonitoringState::new()));

    // Background task to check for stale services
    let cleanup_state = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            sleep(CLEANUP_INTERVAL).await;
            println!("Checking service health");
            cleanup_state.lock().await.check_stale_services();
        }
    });

    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());
    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    println!("Monitoring service starting on {}", addr);

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
