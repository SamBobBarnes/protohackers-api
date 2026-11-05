use std::ops::Add;
use std::time::Duration;
use axum::Json;
use chrono::Local;
use serde::Deserialize;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use uuid::{Timestamp, Uuid};
use crate::helpers::{print_bytes, read_to_line_feed};

#[derive(Deserialize)]
struct IsPrime {
    method: String,
    number: f64,
}

pub async fn json_req_handler(mut stream: TcpStream) -> io::Result<()> {
    let mut close = false;
    let timeout = Duration::from_secs(10);
    let now = Local::now();
    let mut time_to_close = now.add(timeout);
    let id = Uuid::new_v4();
    println!("ip: {}; id: {}",stream.peer_addr()?, id);
    while !close {
        let message = read_to_line_feed(&mut stream).await?;
        if message.len() == 0 {
            if Local::now() >= time_to_close {
                println!("Time to close {}", id);
                close = true;
            }
            continue;
        }
        time_to_close = now.add(timeout);
        println!("------------------\n{}",message.clone());
        let json_result = Json::<IsPrime>::from_bytes(&message.as_bytes());
        if !message.ends_with("\n") {
            return_malformed_response(&mut stream).await?;
            close = true;
        } else if json_result.is_err() {
            return_malformed_response(&mut stream).await?;
            close = true;
        } else {
            let json = json_result.unwrap();
            // evaluate prime
            if json.method != "isPrime" {
                return_malformed_response(&mut stream).await?;
                close = true;
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
    }
    println!("Closing {}", id);
    stream.shutdown().await?;
    Ok(())
}

async fn return_prime_response(stream: &mut TcpStream,is_prime: bool) -> io::Result<()> {
    let is_prime_str = is_prime.to_string();
    let response = format!("{{\"method\":\"isPrime\",\"prime\":{is_prime_str}}}\n");
    println!("returning: {}", response);
    stream.write_all(response.as_bytes()).await?;

    Ok(())
}
async fn return_malformed_response(stream: &mut TcpStream) -> io::Result<()> {
    println!("returning malformed response");
    stream.write_all("malformed\n".as_bytes()).await?;

    Ok(())
}