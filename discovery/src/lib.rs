use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const REGISTER_PROCEDURE: ProcedureId = 1;
// pub const PING_PROCEDURE: ProcedureId = 2;
pub const QUERY_PROCEDURE: ProcedureId = 2;
pub const LIST_PROCEDURE: ProcedureId = 3;

#[derive(Debug, Serializable, Deserializable)]
pub struct RegisterArgs {
    pub name: String,
    pub address: String,
}

// #[derive(Debug, Serializable, Deserializable)]
// pub struct PingArgs {
//     pub address: String,
//     pub port: String,
// }

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryArgs {
    pub name: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryResult {
    pub address: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListArgs {
    pub name: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListResult {
    pub addresses: String,
}

use rpc::{client, Request};
use std::thread;
use tokio::time::{interval, sleep, Duration};

const SYSTEM_ADDRESS: &str = "127.0.0.1:10200";
const PING_INTERVAL: Duration = Duration::from_secs(5);
const BASE_DELAY: Duration = Duration::from_secs(1);
const MAX_DELAY: Duration = Duration::from_secs(60);
const MAX_RETRIES: u32 = 10;

pub fn register(name: String, address: String) {
    let args = RegisterArgs {
        name: name,
        address: address,
    };

    thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(register_with_ping(args));
    });
}

async fn register_with_ping(args: RegisterArgs) {
    let mut interval = interval(PING_INTERVAL);
    let mut retries = 0;
    let mut delay = BASE_DELAY;

    let request = Request {
        procedure_id: REGISTER_PROCEDURE,
        payload: args.serialize(),
    };
    println!("{:?}", request.clone());

    loop {
        match client::send_request(SYSTEM_ADDRESS, request.clone()).await {
            Ok(response) => {
                println!("Response: {}", response.payload);
                interval.tick().await;
                delay = BASE_DELAY;
                retries = 0;
            }
            Err(e) => {
                println!("Failed to send request: {}. Retrying in {:?}", e, delay);
                if retries >= MAX_RETRIES {
                    panic!("Reached maximum retries. Exiting...");
                }

                sleep(delay).await;

                // Exponential backoff with a cap
                delay = std::cmp::min(delay * 2, MAX_DELAY);
                retries += 1;
            }
        }
    }
}

pub async fn query(name: String) -> QueryResult {
    let args = QueryArgs { name: name.clone() };

    let request = Request {
        procedure_id: QUERY_PROCEDURE,
        payload: args.serialize(),
    };

    match client::send_request(SYSTEM_ADDRESS, request.clone()).await {
        Ok(response) => {
            let result =
                QueryResult::deserialize(&response.payload).expect("Failed to deserialize payload");
            println!("Address for {} is {}", name, result.address);
            return result;
        }
        Err(e) => {
            panic!("Failed to send request: {}.", e);
        }
    }
}

pub async fn list(name: String) -> ListResult {
    let args = ListArgs { name: name.clone() };

    let request = Request {
        procedure_id: LIST_PROCEDURE,
        payload: args.serialize(),
    };

    match client::send_request(SYSTEM_ADDRESS, request.clone()).await {
        Ok(response) => {
            let result =
                ListResult::deserialize(&response.payload).expect("Failed to deserialize payload");
            println!("Addresses for {}: {}", name, result.addresses);
            result
        }
        Err(e) => {
            panic!("Failed to send list request: {}.", e);
        }
    }
}
