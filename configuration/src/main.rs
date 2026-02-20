use configuration::{
    DeleteArgs, GetArgs, GetResult, ListArgs, ListResult, SetArgs, WatchArgs, WatchEvent,
    DELETE_PROCEDURE, GET_PROCEDURE, LIST_PROCEDURE, SET_PROCEDURE, SYSTEM_ADDRESS, SYSTEM_NAME,
    WATCH_PROCEDURE,
};
use rpc::{server, Request, Response};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

struct ConfigStore {
    data: HashMap<String, String>,
    watchers: broadcast::Sender<(String, String)>,
}

impl ConfigStore {
    fn new() -> Self {
        let (tx, _) = broadcast::channel(256);
        ConfigStore {
            data: HashMap::new(),
            watchers: tx,
        }
    }
}

mod handlers {
    use super::*;

    pub async fn get(payload: &str, store: &mut ConfigStore) -> Response {
        let args = GetArgs::deserialize(payload).expect("Failed to deserialize payload");
        let value = store.data.get(&args.key).cloned().unwrap_or_default();
        let result = GetResult { value };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn set(payload: &str, store: &mut ConfigStore) -> Response {
        let args = SetArgs::deserialize(payload).expect("Failed to deserialize payload");
        store.data.insert(args.key.clone(), args.value.clone());
        let _ = store.watchers.send((args.key, args.value));
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn delete(payload: &str, store: &mut ConfigStore) -> Response {
        let args = DeleteArgs::deserialize(payload).expect("Failed to deserialize payload");
        store.data.remove(&args.key);
        let _ = store.watchers.send((args.key, String::new()));
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn list(payload: &str, store: &mut ConfigStore) -> Response {
        let args = ListArgs::deserialize(payload).expect("Failed to deserialize payload");
        let keys: Vec<String> = store
            .data
            .keys()
            .filter(|k| k.starts_with(&args.prefix))
            .cloned()
            .collect();
        let result = ListResult {
            keys: keys.join(","),
        };
        Response {
            payload: result.serialize(),
        }
    }

    pub async fn watch(payload: &str, store: &mut ConfigStore) -> Response {
        let args = WatchArgs::deserialize(payload).expect("Failed to deserialize payload");

        // Return current value for the watched key
        // Clients poll this endpoint to detect changes
        let value = store.data.get(&args.key).cloned().unwrap_or_default();
        let event = WatchEvent {
            key: args.key,
            value,
        };
        Response {
            payload: event.serialize(),
        }
    }
}

async fn request_handler(request: Request, shared_state: Arc<Mutex<ConfigStore>>) -> Response {
    let mut store = shared_state.lock().await;
    match request.procedure_id {
        GET_PROCEDURE => handlers::get(&request.payload, &mut store).await,
        SET_PROCEDURE => handlers::set(&request.payload, &mut store).await,
        DELETE_PROCEDURE => handlers::delete(&request.payload, &mut store).await,
        LIST_PROCEDURE => handlers::list(&request.payload, &mut store).await,
        WATCH_PROCEDURE => handlers::watch(&request.payload, &mut store).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(ConfigStore::new()));

    let addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());
    discovery::register(SYSTEM_NAME.to_string(), addr.clone());

    println!("Configuration service starting on {}", addr);

    server::start_server_with_state(
        &addr,
        |request, state| {
            Box::pin(request_handler(request, state))
                as Pin<Box<dyn Future<Output = Response> + Send>>
        },
        store,
    )
    .await
    .expect("Server crashed");
}
