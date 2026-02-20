use echo::{EchoArgs, ECHO_PROCEDURE, SYSTEM_ADDRESS};
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
    server::start_server(SYSTEM_ADDRESS, handler)
        .await
        .expect("Server crashed");
}
