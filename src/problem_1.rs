use axum::Json;
use serde::Deserialize;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::helpers::print_bytes;

#[derive(Deserialize)]
struct IsPrime {
    method: String,
    number: f64,
}

pub async fn json_req_handler(mut stream: TcpStream) -> io::Result<()> {

    let mut buffer = [0; 1024];
    let mut string_result = String::new();
    let mut bytes_read = 0;
    println!("------------------");
    loop{
        print!("loop...:");
        let n = stream.read(&mut buffer[..]).await?;
        print_bytes(buffer.to_vec());
        string_result.push_str(&String::from_utf8_lossy(&buffer[..n]));
        if n == 0 { break; }
        bytes_read += n;
    }
    println!("\n ------ \n{} bytes read and returned to client from {}\n---------\n{}\n---------", bytes_read,stream.peer_addr()?, string_result);

    let json_result = Json::<IsPrime>::from_bytes(string_result.as_bytes());
    if !string_result.ends_with("\n"){
        return_malformed_response(&mut stream).await?;
    } else if json_result.is_err() {
        return_malformed_response(&mut stream).await?;
    } else {
        let json = json_result.unwrap();
        // evaluate prime
        if json.method != "isPrime" {
            return_malformed_response(&mut stream).await?;
        } else {
            if json.number.fract() != 0.0 {
                return_prime_response(&mut stream, false).await?;
            } else {
            let number = json.number as u64;
                let is_prime = primes::is_prime(number);
                return_prime_response(&mut stream,is_prime).await?;
            }
        }
    }

    Ok(())
}

async fn return_prime_response(stream: &mut TcpStream,is_prime: bool) -> io::Result<()> {
    let is_prime_str = is_prime.to_string();
    let response = format!("{{\"method\":\"isPrime\",\"prime\":{is_prime_str}}}\n");
    println!("returning: {}", response);
    stream.write_all(response.as_bytes()).await?;

    stream.shutdown().await?;
    Ok(())
}
async fn return_malformed_response(stream: &mut TcpStream) -> io::Result<()> {
    println!("returning malformed response");
    stream.write_all("malformed\n".as_bytes()).await?;

    stream.shutdown().await?;
    Ok(())
}