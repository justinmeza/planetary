use echo::{EchoArgs, ECHO_PROCEDURE, SYSTEM_ADDRESS, SYSTEM_NAME};
use rpc::{server_v2, Request, Response};
use tokio::time::{timeout, sleep, Duration};
use std::pin::Pin;
use std::future::Future;

const TIMEOUT_DURATION: Duration = Duration::from_secs(5);

async fn handle_echo(args: EchoArgs) -> String {
    sleep(Duration::from_secs(10)).await;
    args.message
}

async fn handle_with_timeout(request: Request) -> Response {
    match timeout(TIMEOUT_DURATION, async {
        match request.procedure_id {
            ECHO_PROCEDURE => {
                let args: EchoArgs = EchoArgs::deserialize(&request.payload).expect("Failed to deserialize");
                Response {
                    payload: handle_echo(args).await,
                }
            }
            _ => Response {
                payload: "Unknown procedure".to_string(),
            },
        }
    }).await {
        Ok(response) => response,
        Err(_) => Response {
            payload: "Operation timed out".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    discovery::register(SYSTEM_NAME.to_string(), SYSTEM_ADDRESS.to_string());
    server_v2::start_server(
        SYSTEM_ADDRESS,
        |request| {
            Box::pin(handle_with_timeout(request))
                as Pin<Box<dyn Future<Output = Response> + Send>>
        },
    )
    .await
    .expect("Server crashed");
}
