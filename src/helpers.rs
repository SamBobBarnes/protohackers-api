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
    // Step 1: Read the exact size prefix (e.g., a 4-byte u32 integer)
    let mut size_buffer = [0u8; 4];
    stream.read_exact(&mut size_buffer).await?;

    // Convert bytes to integer (assuming network/big-endian byte order)
    let message_len = u32::from_be_bytes(size_buffer) as usize;

    // Step 2: Initialize a vector to the exact size and fill it
    let mut payload_buffer = vec![0u8; message_len];
    stream.read_exact(&mut payload_buffer).await?;

    Ok(payload_buffer)
}