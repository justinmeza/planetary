use echo::{EchoArgs, ECHO_PROCEDURE};
use routing::Router;

#[tokio::main]
async fn main() {
    let args = EchoArgs {
        message: "Hello RPC!".to_string(),
    };
    let serialized_args = args.serialize();

    let routing = Router::new();
    let response = routing
        .send_request("echo".to_string(), ECHO_PROCEDURE, &serialized_args)
        .await;
    println!("Response: {}", response.payload);
}
