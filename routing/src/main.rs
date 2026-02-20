use routing::ConnectionPool;
use routing::{RouteArgs, SetStrategyArgs, ROUTE_PROCEDURE, ROUTE_SET_STRATEGY_PROCEDURE};
use rpc::{server, Request, Response};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

const SYSTEM_NAME: &str = "routing";
const SYSTEM_ADDRESS: &str = "127.0.0.1:10300";

mod handlers {
    use super::*;

    pub async fn route(
        payload: &str,
        pools: Arc<Mutex<HashMap<String, ConnectionPool>>>,
    ) -> Response {
        println!("{}", payload);
        let args = RouteArgs::deserialize(&payload).expect("Failed to deserialize payload");

        let mut pools = pools.lock().await;
        let pool = pools
            .entry(args.name.clone())
            .or_insert_with(|| ConnectionPool::new(args.name.clone(), 10));

        match pool.send_request(args.procedure_id, &args.payload).await {
            Ok(response_data) => Response {
                payload: response_data,
            },
            Err(err_msg) => Response { payload: err_msg },
        }
    }

    pub async fn set_strategy(
        payload: &str,
        pools: Arc<Mutex<HashMap<String, ConnectionPool>>>,
    ) -> Response {
        let args =
            SetStrategyArgs::deserialize(&payload).expect("Failed to deserialize payload");
        let mut pools = pools.lock().await;
        for pool in pools.values_mut() {
            pool.set_strategy(args.strategy.clone());
        }
        println!("Routing strategy changed to: {}", args.strategy);
        Response {
            payload: "OK".to_string(),
        }
    }
}

async fn request_handler(
    request: Request,
    shared_state: Arc<Mutex<HashMap<String, ConnectionPool>>>,
) -> Response {
    match request.procedure_id {
        ROUTE_PROCEDURE => handlers::route(&request.payload, shared_state).await,
        ROUTE_SET_STRATEGY_PROCEDURE => {
            handlers::set_strategy(&request.payload, shared_state).await
        }
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let service_pools = Arc::new(Mutex::new(HashMap::new()));
    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());
    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    server::start_server_with_state(
        &addr,
        |request, state| {
            Box::pin(request_handler(request, state))
                as Pin<Box<dyn Future<Output = Response> + Send>>
        },
        service_pools,
    )
    .await
    .expect("Server crashed");
}
