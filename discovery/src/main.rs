use discovery::{
    ListArgs, ListResult, QueryArgs, QueryResult, RegisterArgs, LIST_PROCEDURE, QUERY_PROCEDURE,
    REGISTER_PROCEDURE,
};
use rand::seq::SliceRandom;
use rpc::{server, Request, Response};
use std::collections::{HashMap, HashSet};
// use std::sync::{Arc, Mutex};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

type Name = String;
type Address = String;
const CLEANUP_DURATION: Duration = Duration::from_secs(10);
const SERVER_ADDRESS: &str = "127.0.0.1:10200";

#[derive(Default)]
pub struct Registry {
    registry: HashMap<Name, Vec<Address>>,
    last_ping: HashMap<Address, Instant>,
}

impl Registry {
    // Register a new address or update the last ping time
    fn register(&mut self, name: Name, address: Address) {
        if let Some(time) = self.last_ping.get_mut(&address) {
            *time = Instant::now();
        } else {
            self.registry
                .entry(name)
                .or_insert_with(Vec::new)
                .push(address.clone());
            self.last_ping.insert(address, Instant::now());
        }
    }

    // Randomly retrieve an address for a given name
    fn get_address(&self, name: &Name) -> Option<&Address> {
        self.registry.get(name)?.choose(&mut rand::thread_rng())
    }

    // Return all addresses for a given name
    fn get_all_addresses(&self, name: &Name) -> Vec<&Address> {
        self.registry
            .get(name)
            .map(|addrs| addrs.iter().collect())
            .unwrap_or_default()
    }

    // Remove stale entries based on the last ping time
    fn cleanup_stale(&mut self) {
        let now = Instant::now();
        let stale_addresses: HashSet<_> = self
            .last_ping
            .iter()
            .filter(|&(_, time)| now.duration_since(*time) > CLEANUP_DURATION)
            .map(|(address, _)| address.clone())
            .collect();

        for address in stale_addresses {
            self.last_ping.remove(&address);
            self.registry.retain(|_, v| {
                v.retain(|a| a != &address);
                !v.is_empty()
            });
        }
    }
}

// Handler functions for different procedure calls
mod handlers {
    use super::*;

    pub async fn register(payload: &str, registry: &mut Registry) -> Response {
        println!("{}", payload);
        let args = RegisterArgs::deserialize(&payload).expect("Failed to deserialize payload");
        registry.register(args.name, args.address);
        println!("{:?}", registry.registry);
        Response {
            payload: "OK".to_string(),
        }
    }

    pub async fn query(payload: &str, registry: &mut Registry) -> Response {
        let args = QueryArgs::deserialize(&payload).expect("Failed to deserialize payload");
        match registry.get_address(&args.name) {
            Some(address) => {
                let result = QueryResult {
                    address: address.to_string(),
                };
                Response {
                    payload: result.serialize(),
                }
            }
            None => Response {
                payload: QueryResult {
                    address: "".to_string(),
                }
                .serialize(),
            },
        }
    }

    pub async fn list(payload: &str, registry: &mut Registry) -> Response {
        let args = ListArgs::deserialize(&payload).expect("Failed to deserialize payload");
        let addresses = registry.get_all_addresses(&args.name);
        let joined: Vec<String> = addresses.iter().map(|a| a.to_string()).collect();
        let result = ListResult {
            addresses: joined.join(";"),
        };
        Response {
            payload: result.serialize(),
        }
    }
}

async fn request_handler(request: Request, shared_state: Arc<Mutex<Registry>>) -> Response {
    let mut registry = shared_state.lock().await; //.expect("Failed to lock the mutex");
    match request.procedure_id {
        REGISTER_PROCEDURE => handlers::register(&request.payload, &mut registry).await,
        QUERY_PROCEDURE => handlers::query(&request.payload, &mut registry).await,
        LIST_PROCEDURE => handlers::list(&request.payload, &mut registry).await,
        _ => Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let registry = Arc::new(Mutex::new(Registry::default()));

    // Background cleanup task
    let cleanup_registry = Arc::clone(&registry);
    tokio::spawn(async move {
        loop {
            sleep(CLEANUP_DURATION).await;
            println!("Cleaning registry");
            cleanup_registry.lock().await.cleanup_stale();
            println!("{:?}", cleanup_registry.lock().await.registry);
        }
    });

    // let boxed_handler = |request, state| {
    //     let future: Pin<Box<dyn Future<Output = Response> + Send>> = Box::pin(request_handler(request, state));
    //     future
    // };
    server::start_server_with_state(
        SERVER_ADDRESS,
        |request, state| {
            Box::pin(request_handler(request, state))
                as Pin<Box<dyn Future<Output = Response> + Send>>
        },
        registry,
    )
    .await
    .expect("Server crashed");
}

// use discovery::{QueryArgs, QueryResult, RegisterArgs, QUERY_PROCEDURE, REGISTER_PROCEDURE};
// use rand::seq::SliceRandom;
// use rpc::{server, Request, Response};
// use std::collections::{HashMap, HashSet};
// use std::sync::{Arc, Mutex};
// use tokio::time::{sleep, Duration, Instant};

// type Name = String;
// type Address = String;
// const CLEANUP_DURATION: Duration = Duration::from_secs(10);

// #[derive(Default)]
// struct Registry {
//     registry: HashMap<Name, Vec<Address>>,
//     last_ping: HashMap<Address, Instant>,
// }

// impl Registry {
//     fn register(&mut self, name: Name, address: Address) {
//         if let Some(time) = self.last_ping.get_mut(&address) {
//             *time = Instant::now();
//         } else {
//             self.registry
//                 .entry(name)
//                 .or_insert_with(Vec::new)
//                 .push(address.clone());
//             self.last_ping.insert(address, Instant::now());
//         }
//     }

//     fn get_address(&self, name: &Name) -> Option<&Address> {
//         if let Some(addresses) = self.registry.get(name) {
//             if !addresses.is_empty() {
//                 let chosen = addresses.choose(&mut rand::thread_rng())?;
//                 return Some(&chosen);
//             }
//         }
//         None
//     }

//     // fn ping(&mut self, address: Address) {
//     //     if let Some(time) = self.last_ping.get_mut(&address) {
//     //         *time = Instant::now();
//     //     }
//     // }

//     fn cleanup_stale(&mut self) {
//         let now = Instant::now();
//         let mut to_remove = HashSet::new();

//         for (address, time) in &self.last_ping {
//             if now.duration_since(*time) > CLEANUP_DURATION {
//                 to_remove.insert(address.clone());
//             }
//         }

//         for address in &to_remove {
//             self.last_ping.remove(address);
//             self.registry.retain(|_, v| {
//                 v.retain(|a| a != address);
//                 !v.is_empty()
//             });
//         }
//     }
// }

// fn process_request(request: Request, registry: &mut Registry) -> Response {
//     match request.procedure_id {
//         REGISTER_PROCEDURE => handle_register(&request.payload, registry),
//         // PING_PROCEDURE => handle_ping(&request.payload, registry),
//         QUERY_PROCEDURE => handle_query(&request.payload, registry),
//         _ => Response {
//             payload: "Unknown procedure".to_string(),
//         },
//     }
// }

// fn handle_register(payload: &str, registry: &mut Registry) -> Response {
//     println!("{}", payload);
//     let args = RegisterArgs::deserialize(&payload).expect("Failed to deserialize payload");
//     registry.register(args.name, format!("{}:{}", args.address, args.port));
//     println!("{:?}", registry.registry);
//     Response {
//         payload: "OK".to_string(),
//     }
// }

// // fn handle_ping(payload: &str, registry: &mut Registry) -> Response {
// //     let args = PingArgs::deserialize(&payload).expect("Failed to deserialize payload");
// //     registry.ping(format!("{}:{}", args.address, args.port));
// //     Response {
// //         payload: "Pinged".to_string(),
// //     }
// // }

// fn handle_query(payload: &str, registry: &mut Registry) -> Response {
//     let args = QueryArgs::deserialize(&payload).expect("Failed to deserialize payload");
//     match registry.get_address(&args.name) {
//         Some(address) => {
//             let parts: Vec<&str> = address.splitn(2, ':').collect();
//             let result = QueryResult {
//                 address: parts[0].to_string(),
//                 port: parts[1].to_string(),
//             };
//             Response {
//                 payload: result.serialize(),
//             }
//         }
//         None => Response {
//             payload: "System not found".to_string(),
//         },
//     }
// }

// fn request_handler(request: Request, shared_state: Arc<Mutex<Registry>>) -> Response {
//     let mut registry = shared_state.lock().expect("Failed to lock the mutex");
//     process_request(request, &mut registry)
// }

// #[tokio::main]
// async fn main() {
//     let registry = Arc::new(Mutex::new(Registry::default()));

//     // Background cleanup task
//     let cleanup_registry = Arc::clone(&registry);
//     tokio::spawn(async move {
//         loop {
//             sleep(CLEANUP_DURATION).await;
//             println!("Cleaning registry");
//             cleanup_registry.lock().unwrap().cleanup_stale();
//             println!("{:?}", cleanup_registry.lock().unwrap().registry);
//         }
//     });

//     server::start_server_with_state("127.0.0.1:10200", request_handler, registry)
//         .await
//         .expect("Server crashed");
// }

// // use tokio::{time::{Instant, Duration, sleep}};
// // use std::collections::{HashMap, HashSet};
// // use std::sync::{Arc, Mutex};
// // use rand::seq::SliceRandom;
// // use rpc::{Request, Response, server};
// // use discovery::{RegisterArgs, PingArgs, QueryArgs, QueryResult, REGISTER_PROCEDURE, PING_PROCEDURE, QUERY_PROCEDURE};

// // type Name = String;
// // type Address = String;

// // #[derive(Default)]
// // struct Registry {
// //     registry: HashMap<Name, Vec<Address>>,
// //     last_ping: HashMap<Address, Instant>,
// // }

// // impl Registry {
// //     fn register(&mut self, name: Name, address: Address) {
// //         self.registry.entry(name).or_insert_with(Vec::new).push(address.clone());
// //         self.last_ping.insert(address, Instant::now());
// //     }

// //     fn get_address(&self, name: &Name) -> Option<Address> {
// //         if let Some(addresses) = self.registry.get(name) {
// //             if !addresses.is_empty() {
// //                 let chosen = addresses.choose(&mut rand::thread_rng())?; // Randomly choose an address
// //                 return Some(chosen.clone());
// //             }
// //         }
// //         None
// //     }

// //     fn ping(&mut self, address: Address) {
// //         if let Some(time) = self.last_ping.get_mut(&address) {
// //             *time = Instant::now();
// //         }
// //     }

// //     fn cleanup_stale(&mut self) {
// //         let now = Instant::now();
// //         let mut to_remove = HashSet::new();

// //         for (address, time) in &self.last_ping {
// //             if now.duration_since(*time) > Duration::from_secs(10) {
// //                 to_remove.insert(address.clone());
// //             }
// //         }

// //         for address in &to_remove {
// //             self.last_ping.remove(address);
// //             self.registry.retain(|_, v| {
// //                 v.retain(|a| a != address);
// //                 !v.is_empty()
// //             });
// //         }

// //         // for address in to_remove {
// //         //     self.last_ping.remove(&address);
// //         //     self.registry.retain(|_, v| {
// //         //         v.retain(|a| a != &address);
// //         //         !v.is_empty()
// //         //     });
// //         // }
// //     }
// // }

// // // trait Lock {
// // //     fn lock(&self) -> std::sync::MutexGuard<'_, Registry>;
// // // }

// // // impl Lock for Arc<Mutex<Registry>> {
// // //     fn lock(&self) -> std::sync::MutexGuard<'_, Registry> {
// // //         // self.lock().unwrap()
// // //     }
// // // }

// // fn handler(request: Request, shared_state: Arc<Mutex<Registry>>) -> Response
// // {
// //     // let mut registry = shared_state.lock().unwrap();
// //     // Lock the mutex to get mutable access to the shared data
// //     let mut registry = shared_state.lock().expect("Failed to lock the mutex");
// //     match request.procedure_id {
// //         REGISTER_PROCEDURE => {
// //             println!("{}", request.payload);
// //             let args =
// //                 RegisterArgs::deserialize(&request.payload).expect("Failed to deserialize payload");
// //             registry.register(args.name, format!("{}:{}", args.address, args.port));
// //             println!("{:?}", registry.registry);
// //             Response {
// //                 payload: "OK".to_string(),
// //             }
// //         }
// //         PING_PROCEDURE => {
// //             let args = PingArgs::deserialize(&request.payload).expect("Failed to deserialize payload");
// //             registry.ping(format!("{}:{}", args.address, args.port));
// //             Response {
// //                 payload: "Pinged".to_string(),
// //             }
// //         }
// //         QUERY_PROCEDURE => {
// //             let args = QueryArgs::deserialize(&request.payload).expect("Failed to deserialize payload");
// //             match registry.get_address(&args.name) {
// //                 Some(address) => {
// //                     let parts: Vec<&str> = address.splitn(2, ':').collect();
// //                     let result = QueryResult {
// //                         address: parts[0].to_string(),
// //                         port: parts[1].to_string(),
// //                     };
// //                     Response {
// //                         payload: result.serialize(),
// //                     }
// //                 },
// //                 None => {
// //                     Response {
// //                         payload: "System not found".to_string(),
// //                     }
// //                 },
// //             }
// //         }
// //         _ => Response {
// //             payload: "Unknown procedure".to_string(),
// //         },
// //     }
// // }

// // #[tokio::main]
// // async fn main() {
// //     // let listener = TcpListener::bind("127.0.0.1:10200").await.unwrap();
// //     let registry = Arc::new(Mutex::new(Registry::default()));

// //     // Background cleanup task
// //     let cleanup_registry = Arc::clone(&registry);
// //     tokio::spawn(async move {
// //         loop {
// //             sleep(Duration::from_secs(10)).await;
// //             println!("Cleaning registry");
// //             cleanup_registry.lock().unwrap().cleanup_stale();
// //             println!("{:?}", cleanup_registry.lock().unwrap().registry);
// //         }
// //     });

// //     server::start_server_with_state("127.0.0.1:10200", handler, registry)
// //         .await
// //         .expect("Server crashed");

// //     // loop {
// //     //     let (mut client_socket, _) = listener.accept().await.unwrap();
// //     //     let registry_clone = Arc::clone(&registry);

// //     //     tokio::spawn(async move {
// //     //         let mut buf = vec![0u8; 1024];
// //     //         let _nbytes = client_socket.read(&mut buf).await.unwrap();

// //     //         let request_payload = String::from_utf8_lossy(&buf).trim().to_string();
// //     //         let request: Request = Request::deserialize(&request_payload).expect("Failed to deserialize request");

// //     //         let response_payload: Payload = match request.procedure_id {
// //     //             REGISTER_PROCEDURE => {
// //     //                 let (name, address) = Payload::deserialize(&request.payload).expect("Failed to deserialize payload");
// //     //                 registry_clone.lock().unwrap().register(name, address);
// //     //                 "OK".serialize()
// //     //             },
// //     //             QUERY_PROCEDURE => {
// //     //                 let name = Payload::deserialize(&request.payload).expect("Failed to deserialize payload");
// //     //                 match registry_clone.lock().unwrap().get_address(&name) {
// //     //                     Some(address) => address.serialize(),
// //     //                     None => "Service not found".serialize(),
// //     //                 }
// //     //             },
// //     //             PING_PROCEDURE => {
// //     //                 let address = Payload::deserialize(&request.payload).expect("Failed to deserialize payload");
// //     //                 registry_clone.lock().unwrap().ping(address);
// //     //                 "Pinged".serialize()
// //     //             },
// //     //             _ => "Invalid procedure ID".serialize(),
// //     //         };

// //     //         let response = Response { payload: response_payload };
// //     //         let res_bytes = response.serialize();
// //     //         client_socket.write_all(res_bytes.as_bytes()).await.unwrap();
// //     //     });
// //     // }
// // }
