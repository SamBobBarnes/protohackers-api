use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::helpers::print_bytes;

pub async fn json_req_handler(mut stream: TcpStream) -> io::Result<()> {

    let mut buffer = [0; 1024];
    let mut string_result = String::new();
    let mut bytes_read = 0;
    println!("------------------");
    loop{
        let n = stream.read(&mut buffer[..]).await?;
        string_result.push_str(&String::from_utf8_lossy(&buffer[..n]));
        if n == 0 { break; }
        bytes_read += n;
    }
    println!("\n{} bytes read and returned to client from {}\n\n{}", bytes_read,stream.peer_addr()?, string_result);

    stream.shutdown().await?;
    Ok(())
}