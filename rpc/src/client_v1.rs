use crate::{Request, Response};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use rand::Rng;
use once_cell::sync::Lazy;

// const MAX_RETRIES: usize = 3;
// const BASE_DELAY_MS: u64 = 100;
// const JITTER_MS: u64 = 50;

static MAX_RETRIES: Lazy<usize> = Lazy::new(|| {
    std::env::var("MAX_RETRIES")
        .map(|val| val.parse().expect("MAX_RETRIES must be a number"))
        .unwrap_or(3)
});

static BASE_DELAY_MS: Lazy<u64> = Lazy::new(|| {
    std::env::var("BASE_DELAY_MS")
        .map(|val| val.parse().expect("BASE_DELAY_MS must be a number"))
        .unwrap_or(100)
});

static JITTER_MS: Lazy<u64> = Lazy::new(|| {
    std::env::var("JITTER_MS")
        .map(|val| val.parse().expect("JITTER_MS must be a number"))
        .unwrap_or(50)
});

pub async fn send_request(server_addr: &str, request: Request) -> io::Result<Response> {
    let serialized = format!("{}:{}\n", request.procedure_id, request.payload);

    for attempt in 1..=*MAX_RETRIES {
        match attempt_request(server_addr, &serialized).await {
            Ok(response) => return Ok(response),
            Err(err) => {
                if attempt == *MAX_RETRIES {
                    return Err(err);
                }
                eprintln!("Retrying request");
                let jittered_delay = *BASE_DELAY_MS + rand::thread_rng().gen_range(0..*JITTER_MS);
                sleep(Duration::from_millis(jittered_delay)).await;
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "Reached max retries"))
}

async fn attempt_request(server_addr: &str, serialized: &str) -> io::Result<Response> {
    let mut stream = TcpStream::connect(server_addr).await?;
    println!("Sending: {}", serialized);
    stream.write_all(serialized.as_bytes()).await?;

    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    let response_data = String::from_utf8_lossy(&buffer[..n]);

    match &*response_data {
        "Operation timed out" => {
            Err(io::Error::new(io::ErrorKind::TimedOut, "Operation timed out"))
        }
        "Unknown procedure" => {
            Err(io::Error::new(io::ErrorKind::Unsupported, "Unknown procedure"))
        }
        "Service not available" => {
            Err(io::Error::new(io::ErrorKind::NotFound, "Service not available"))
        }
        _ => {
            Ok(Response {
                payload: response_data.to_string(),
            })
        }
    }
}
