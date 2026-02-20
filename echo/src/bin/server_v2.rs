use echo::{EchoArgs, ECHO_PROCEDURE, SYSTEM_ADDRESS, SYSTEM_NAME};
use rpc::{server_v1, Request, Response};
use std::{thread, time::Duration};

fn handle_echo(args: EchoArgs) -> String {
    thread::sleep(Duration::from_secs(60));
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
    discovery::register(SYSTEM_NAME.to_string(), SYSTEM_ADDRESS.to_string());
    server_v1::start_server(SYSTEM_ADDRESS, handler)
        .await
        .expect("Server crashed");
}
