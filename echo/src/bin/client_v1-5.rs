use echo::{EchoArgs, ECHO_PROCEDURE, SYSTEM_NAME};
use rpc::{client, Request};

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

    let address = discovery::query(SYSTEM_NAME.to_string()).await.address;

    let response = client::send_request(&address, request)
        .await
        .expect("Failed to get response");
    println!("Response: {}", response.payload);
}
