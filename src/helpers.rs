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

pub async fn read_dynamic_message(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut buf = [0u8; 1024];
    let mut buf_n = 0;
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                println!("Read {} bytes from stream", n);
                buf_n = n;
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
    Ok(buf[..buf_n].to_vec())
}