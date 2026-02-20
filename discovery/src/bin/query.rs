// use discovery::{QueryArgs, QueryResult, QUERY_PROCEDURE};
// use rpc::{client, Request};

// const DISCOVERY_ADDRESS: &str = "127.0.0.1:10200";
const SYSTEM_NAME: &str = "my_system";

#[tokio::main]
async fn main() {
    // let args = QueryArgs {
    //     name: SYSTEM_NAME.to_string(),
    // };

    // let request = Request {
    //     procedure_id: QUERY_PROCEDURE,
    //     payload: args.serialize(),
    // };

    // match client::send_request(DISCOVERY_ADDRESS, request.clone()).await {
    //     Ok(response) => {
    //         let result =
    //             QueryResult::deserialize(&response.payload).expect("Failed to deserialize payload");
    //         println!(
    //             "Address for {} is {}:{}",
    //             SYSTEM_NAME, result.address, result.port
    //         );
    //     }
    //     Err(e) => {
    //         println!("Failed to send request: {}.", e);
    //     }
    // }
    discovery::query(SYSTEM_NAME.to_string()).await;
}
