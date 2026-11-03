pub mod problem_0;
pub mod problem_1;
pub mod helpers;

use axum::{Json, Router, response::IntoResponse, routing::post};

use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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

async fn handle_client(stream: TcpStream)-> io::Result<()>  {

    // problem_0::echo(stream).await?;
    problem_1::json_req_handler(stream).await?;
    Ok(())
    
}
