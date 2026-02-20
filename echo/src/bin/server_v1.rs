use echo::{EchoArgs, ECHO_PROCEDURE, SYSTEM_ADDRESS, SYSTEM_NAME};
use rpc::{server, Request, Response};

fn handle_echo(args: EchoArgs) -> String {
    args.message
}

fn handler(request: Request) -> Response {
    match request.procedure_id {
        ECHO_PROCEDURE => {
            let args: EchoArgs =
                EchoArgs::deserialize(&request.payload).expect("Failed to deserialize");
            Response {
                payload: handle_echo(args),
            }
        }
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
    discovery::register(SYSTEM_NAME.to_string(), addr.clone());
    server::start_server(&addr, handler)
        .await
        .expect("Server crashed");
}
