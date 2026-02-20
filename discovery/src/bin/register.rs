// use discovery::{RegisterArgs};
// use rpc::{client, Request};
// use tokio::time::{interval, sleep, Duration};

// const DISCOVERY_ADDRESS: &str = "127.0.0.1:10200";
const SYSTEM_NAME: &str = "my_system";
const SYSTEM_ADDRESS: &str = "127.0.0.1:10201";
// const PING_INTERVAL: u64 = 5;
// const BASE_DELAY: Duration = Duration::from_secs(1);
// const MAX_DELAY: Duration = Duration::from_secs(60);
// const MAX_RETRIES: u32 = 10;

#[tokio::main]
async fn main() {
    // let args = RegisterArgs {
    //     name: SYSTEM_NAME.to_string(),
    //     address: SYSTEM_ADDRESS.to_string(),
    //     port: SYSTEM_PORT.to_string(),
    // };

    discovery::register(SYSTEM_NAME.to_string(), SYSTEM_ADDRESS.to_string());

    loop {}
    // let register_args = RegisterArgs {
    //     name: SYSTEM_NAME.to_string(),
    //     address: SYSTEM_ADDRESS.to_string(),
    //     port: SYSTEM_PORT.to_string(),
    // };

    // // let register_response = send_request(REGISTER_PROCEDURE, register_args.serialize()).await;
    // // println!("Response: {}", register_response.payload);

    // // Now, periodically ping the discovery system.

    // let mut interval = interval(Duration::from_secs(PING_INTERVAL));
    // let mut retries = 0;
    // let mut delay = BASE_DELAY;

    // let args = RegisterArgs {
    //     name: SYSTEM_NAME.to_string(),
    //     address: SYSTEM_ADDRESS.to_string(),
    //     port: SYSTEM_PORT.to_string(),
    // };

    // let request = Request {
    //     procedure_id: REGISTER_PROCEDURE,
    //     payload: args.serialize(),
    // };

    // loop {
    //     match client::send_request(DISCOVERY_ADDRESS, request.clone()).await {
    //         Ok(response) => {
    //             println!("Response: {}", response.payload);
    //             interval.tick().await;
    //             delay = BASE_DELAY;
    //             retries = 0;
    //             // break;
    //         }
    //         Err(e) => {
    //             println!("Failed to send request: {}. Retrying in {:?}", e, delay);
    //             if retries >= MAX_RETRIES {
    //                 panic!("Reached maximum retries. Exiting...");
    //             }

    //             sleep(delay).await;

    //             // Exponential backoff with a cap
    //             delay = std::cmp::min(delay * 2, MAX_DELAY);
    //             retries += 1;
    //         }
    //     }
    // }

    // loop {
    // loop {
    //     match client::send_request(DISCOVERY_ADDRESS, request.clone()).await {
    //         Ok(response) => {
    //             println!("Response: {}", response.payload);
    //             interval.tick().await;
    //             delay = BASE_DELAY;
    //             retries = 0;
    //             // break;
    //         },
    //         Err(e) => {
    //             println!("Failed to send request: {}. Retrying in {:?}", e, delay);
    //             if retries >= MAX_RETRIES {
    //                 panic!("Reached maximum retries. Exiting...");
    //             }

    //             sleep(delay).await;

    //             // Exponential backoff with a cap
    //             delay = std::cmp::min(delay * 2, MAX_DELAY);
    //             retries += 1;
    //         }
    //     }
    // }

    // let response = send_request(REGISTER_PROCEDURE, args.serialize()).await;
    // println!("Response: {}", response.payload);

    // interval.tick().await;
    // let ping_args = PingArgs {
    //     address: SYSTEM_ADDRESS.to_string(),
    //     port: SYSTEM_PORT.to_string(),
    // };
    // let ping_response = send_request(PING_PROCEDURE, ping_args.serialize()).await;
    // println!("Response: {}", ping_response.payload);
    // }
}

// async fn send_request(procedure_id: ProcedureId, payload: String) -> rpc::Response {
//     let request = Request {
//         procedure_id,
//         payload: payload,
//     };

//     let mut retries = 0;
//     let mut delay = BASE_DELAY;

//     loop {
//         match client::send_request(DISCOVERY_ADDRESS, request.clone()).await {
//             Ok(response) => return response,
//             Err(e) => {
//                 println!("Failed to send request: {}. Retrying in {:?}", e, delay);
//                 if retries >= MAX_RETRIES {
//                     panic!("Reached maximum retries. Exiting...");
//                 }

//                 sleep(delay).await;

//                 // Exponential backoff with a cap
//                 delay = std::cmp::min(delay * 2, MAX_DELAY);
//                 retries += 1;
//             }
//         }
//     }

// //     client::send_request(DISCOVERY_ADDRESS, request)
// //         .await
// //         .expect("Failed to get response")
// }

// use tokio::net::TcpStream;
// use normalization::{Serializable};
// use rpc::{Request, client};
// use discovery::{RegisterArgs, PingArgs, REGISTER_PROCEDURE, PING_PROCEDURE};

// #[tokio::main]
// async fn main() {
//     let system_name = "my_system".to_string();
//     let system_address = "127.0.0.1".to_string();
//     let system_port = "10201".to_string();

//     let register_args = RegisterArgs{
//         name: system_name.clone(),
//         address: system_address.clone(),
//         port: system_port.clone(),
//     };

//     let register_request = Request {
//         procedure_id: REGISTER_PROCEDURE,
//         payload: register_args.serialize(),
//     };

//     let register_response = client::send_request("127.0.0.1:10200", register_request)
//         .await
//         .expect("Failed to get response");
//     println!("Response: {}", register_response.payload);

//     // Now, periodically ping the discovery system.
//     let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
//     loop {
//         interval.tick().await;
//         let ping_args = PingArgs {
//             address: system_address.clone(),
//             port: system_port.clone(),
//         };
//         let ping_request = Request {
//             procedure_id: PING_PROCEDURE,
//             payload: ping_args.serialize(),
//         };
//         let ping_response = client::send_request("127.0.0.1:10200", ping_request)
//             .await
//             .expect("Failed to get response");
//         println!("Response: {}", ping_response.payload);
//     }
// }
