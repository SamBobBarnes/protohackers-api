use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::helpers::print_bytes;

pub async fn echo(mut stream: TcpStream) -> io::Result<()>  {
    let mut buffer = [0; 1024];
    let mut bytes_read = 0;
    println!("------------------");
    loop{
        let n = stream.read(&mut buffer[..]).await?;
        stream.write_all(&buffer[..n]).await?;
        if n == 0 { break; }
        print_bytes(buffer.to_vec());
        bytes_read += n;
    }
    println!("\n{} bytes read and returned to client from {}", bytes_read,stream.peer_addr()?);

    stream.shutdown().await?;
    Ok(())
}