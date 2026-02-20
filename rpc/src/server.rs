// use std::sync::{Arc, Mutex};
use crate::{Request, Response};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn start_server(
    addr: &str,
    handler: impl Fn(Request) -> Response + Send + Sync + 'static + Clone,
) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let handler = handler.clone();

        tokio::spawn(async move {
            loop {
                let mut buffer = vec![0u8; 1024];
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => {
                        // Client closed the connection or no data was read.
                        break;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                };
                // let n = socket
                //     .read(&mut buffer)
                //     .await
                //     .expect("Failed to read from socket");
                let data = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", data);

                let parts: Vec<&str> = data.splitn(2, ':').collect();
                let request = Request {
                    procedure_id: parts[0].parse().unwrap(),
                    payload: parts[1].trim().to_string(),
                };

                let response = handler(request);
                println!("Sending: {}", response.payload);
                socket
                    .write_all(response.payload.as_bytes())
                    .await
                    .expect("Failed to write to socket");
            }
        });
    }
}

// pub async fn start_server_keep_alive(
//     addr: &str,
//     handler: impl Fn(Request) -> Response + Send + Sync + 'static + Clone,
// ) -> io::Result<()> {
//     let listener = TcpListener::bind(addr).await?;

//     let (mut socket, _) = listener.accept().await?;
//     let handler = handler.clone();

//     loop {
//         let mut buffer = vec![0u8; 1024];
//         let n = socket
//             .read(&mut buffer)
//             .await
//             .expect("Failed to read from socket");
//         let data = String::from_utf8_lossy(&buffer[..n]);

//         let parts: Vec<&str> = data.splitn(2, ':').collect();
//         let request = Request {
//             procedure_id: parts[0].parse().unwrap(),
//             payload: parts[1].trim().to_string(),
//         };

//         let response = handler(request);
//         socket
//             .write_all(response.payload.as_bytes())
//             .await
//             .expect("Failed to write to socket");
//         }
// }

// pub async fn start_server_with_state<T: Send + 'static>(
//     addr: &str,
//     handler: impl Fn(Request, Arc<Mutex<T>>) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync + 'static + Clone,
//     shared_state: Arc<Mutex<T>>,
// ) -> io::Result<()> {
//     let listener = TcpListener::bind(addr).await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;
//         let handler = handler.clone();
//         let shared_state = shared_state.clone();

//         tokio::spawn(async move {
//             let mut buffer = vec![0u8; 1024];
//             let n = socket
//                 .read(&mut buffer)
//                 .await
//                 .expect("Failed to read from socket");
//             let data = String::from_utf8_lossy(&buffer[..n]);
//             println!("Recieving: {}", data);

//             let parts: Vec<&str> = data.splitn(2, ':').collect();
//             let request = Request {
//                 procedure_id: parts[0].parse().unwrap(),
//                 payload: parts[1].trim().to_string(),
//             };

//             let response = handler(request, shared_state).await;
//             socket
//                 .write_all(response.payload.as_bytes())
//                 .await
//                 .expect("Failed to write to socket");
//         });
//     }
// }

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_server_with_state<T: Send + 'static>(
    addr: &str,
    handler: impl Fn(Request, Arc<Mutex<T>>) -> Pin<Box<dyn Future<Output = Response> + Send>>
        + Send
        + Sync
        + 'static
        + Clone,
    shared_state: Arc<Mutex<T>>,
) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let handler = handler.clone();
        let shared_state = shared_state.clone();

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024];

            // Loop to keep reading from the socket
            loop {
                match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => {
                        // Socket was closed gracefully
                        break;
                    }
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buffer[..n]);
                        println!("Receiving: {}", data);

                        let parts: Vec<&str> = data.splitn(2, ':').collect();
                        let request = Request {
                            procedure_id: parts[0].parse().unwrap(),
                            payload: parts[1].trim().to_string(),
                        };

                        let response = handler(request, shared_state.clone()).await;
                        if socket.write_all(response.payload.as_bytes()).await.is_err() {
                            println!("Failed to write to socket");
                            break;
                        }
                    }
                    Err(_) => {
                        println!("Failed to read from socket");
                        break;
                    }
                }
            }
        });
    }
}
