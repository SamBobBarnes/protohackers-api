use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub fn print_bytes(buffer: Vec<u8>) {
    let string_result = String::from_utf8(buffer.to_vec());
    if string_result.is_ok() {
        print!("{}", string_result.unwrap());
    } else {
        println!("Invalid UTF-8 data");
    }
}

pub async fn read_to_line_feed(stream: &mut TcpStream) -> io::Result<String> {
    let mut message = String::new();
    let mut buf = [0u8; 1024];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                println!("Read {} bytes from stream", n);
                message.push_str(&String::from_utf8_lossy(&buf[..n]));
                if message.ends_with("\n") {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
    Ok(message)
}