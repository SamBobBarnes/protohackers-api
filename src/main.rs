use std::collections::HashMap;
use axum::{Json, Router, response::IntoResponse, routing::post};
use axum::body::{Bytes, HttpBody};
use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use axum::response::Response;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;

#[tokio::main]
async fn main() -> io::Result<()>  {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    loop {
        // Accept a new connection
        let (stream, _addr) = listener.accept().await?;

        // Spawn a new task to handle the connection concurrently
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(mut stream: TcpStream)-> io::Result<()>  {

    let mut buffer = [0; 1024];
    let mut bytes_read = 0;
    let mut n = stream.read(&mut buffer).await? ;
    bytes_read += n;
    while n == 1024{
        stream.write_all(&buffer[..n]).await?;
        n = stream.read(&mut buffer[..]).await?;
        bytes_read += n;
    }
    stream.write_all(&buffer[..n]).await?;
    println!("{} bytes read and returned to client from {}", bytes_read,stream.peer_addr()?);

    Ok(())
}