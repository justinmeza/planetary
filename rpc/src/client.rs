use crate::{Request, Response};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send_request(server_addr: &str, request: Request) -> io::Result<Response> {
    let mut stream = TcpStream::connect(server_addr).await?;
    let serialized = format!("{}:{}\n", request.procedure_id, request.payload);
    println!("Sending: {}", serialized);
    stream.write_all(serialized.as_bytes()).await?;

    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    let response_data = String::from_utf8_lossy(&buffer[..n]);

    Ok(Response {
        payload: response_data.to_string(),
    })
}
