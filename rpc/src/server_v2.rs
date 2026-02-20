use crate::{Request, Response};
use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};
use tokio::sync::{Semaphore, Mutex};
use once_cell::sync::Lazy;

static MAX_CONCURRENT_REQUESTS: Lazy<usize> = Lazy::new(|| {
    std::env::var("MAX_CONCURRENT_REQUESTS")
        .map(|val| val.parse().expect("MAX_CONCURRENT_REQUESTS must be a number"))
        .unwrap_or(100)
});

static MAX_QUEUED_REQUESTS: Lazy<usize> = Lazy::new(|| {
    std::env::var("MAX_QUEUED_REQUESTS")
        .map(|val| val.parse().expect("MAX_QUEUED_REQUESTS must be a number"))
        .unwrap_or(200)
});

pub async fn start_server(
    addr: &str,
    handler: impl Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>>
        + Send
        + Sync
        + 'static
        + Clone,
) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let semaphore = Arc::new(Semaphore::new(*MAX_CONCURRENT_REQUESTS));
    let queued_count = Arc::new(Mutex::new(0usize));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let handler = handler.clone();
        let semaphore = semaphore.clone();
        let queued_count = queued_count.clone();

        if *queued_count.lock().await >= *MAX_QUEUED_REQUESTS {
            continue; // Drop new connections if we've reached our queue limit
        }

        *queued_count.lock().await += 1;
        println!("{} queued", *queued_count.lock().await);

        tokio::spawn(async move {
            let permit = semaphore.acquire().await; // Block until a permit is acquired

            let mut buffer = vec![0u8; 1024];

            loop {
                match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => {
                        break;
                    }
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buffer[..n]);
                        println!("Received: {}", data);

                        let parts: Vec<&str> = data.splitn(2, ':').collect();
                        let request = Request {
                            procedure_id: parts[0].parse().unwrap(),
                            payload: parts[1].trim().to_string(),
                        };

                        let response = handler(request).await;
                        println!("Sending: {}", response.payload);
                        if socket.write_all(response.payload.as_bytes()).await.is_err() {
                            println!("Failed to write to socket");
                            break;
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                };
            }

            *queued_count.lock().await -= 1;
            println!("{} queued", *queued_count.lock().await);
            drop(permit); // Release the semaphore permit
        });
    }
}

// const MAX_CONCURRENT_TASKS: usize = 100;
// const MAX_QUEUED_TASKS: usize = 200;

// pub async fn start_server(
//     addr: &str,
//     handler: impl Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>>
//         + Send
//         + Sync
//         + 'static
//         + Clone,
// ) -> io::Result<()> {
//     let listener = TcpListener::bind(addr).await?;

//     let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS));
//     let queued_count = Arc::new(Mutex::new(0usize));

//     loop {
//         let (socket, _) = listener.accept().await?;
//         let handler = handler.clone();
//         let semaphore = semaphore.clone();
//         let queued_count = queued_count.clone();

//         if *queued_count.lock().await >= MAX_QUEUED_TASKS {
//             continue; // Drop new connections if we've reached our queue limit
//         }

//         *queued_count.lock().await += 1;
//         println!("{} queued", *queued_count.lock().await);

//         tokio::spawn(async move {
//             let permit = semaphore.acquire().await; // Block until a permit is acquired

//             let _ = handle_connection(socket, &handler).await;

//             *queued_count.lock().await -= 1;
//             println!("{} queued", *queued_count.lock().await);
//             drop(permit); // Release the semaphore permit
//         });
//     }
// }

// async fn handle_connection(
//     mut socket: TcpStream,
//     handler: &impl Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>>,
// ) -> io::Result<()> {
//     loop {
//         let mut buffer = vec![0u8; 1024];
//         let n = match socket.read(&mut buffer).await {
//             Ok(n) if n == 0 => {
//                 break;
//             }
//             Ok(n) => n,
//             Err(e) => {
//                 eprintln!("Failed to read from socket: {}", e);
//                 break;
//             }
//         };
//         let data = String::from_utf8_lossy(&buffer[..n]);
//         println!("Received: {}", data);

//         let parts: Vec<&str> = data.splitn(2, ':').collect();
//         let request = Request {
//             procedure_id: parts[0].parse().unwrap(),
//             payload: parts[1].trim().to_string(),
//         };

//         let response = handler(request).await;
//         println!("Sending: {}", response.payload);
//         socket.write_all(response.payload.as_bytes()).await?;
//     }
//     Ok(())
// }
