use echo::{EchoArgs, ECHO_PROCEDURE, SYSTEM_ADDRESS};
use rpc::{client_v1, Request};

#[tokio::main]
async fn main() {
    let args = EchoArgs {
        message: "Hello RPC!".to_string(),
    };
    let serialized_args = args.serialize();

    let request = Request {
        procedure_id: ECHO_PROCEDURE,
        payload: serialized_args,
    };

    let response = client_v1::send_request(SYSTEM_ADDRESS, request)
        .await
        .expect("Failed to get response");
    println!("Response: {}", response.payload);
}
