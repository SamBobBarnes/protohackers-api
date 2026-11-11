pub mod problem_0;
pub mod problem_1;
pub mod helpers;
pub mod problem_2;
mod problem_3;

use std::sync::Arc;
use axum::{Json, Router, response::IntoResponse, routing::post};

use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use crate::problem_3::room::ChatRoom;

#[tokio::main]
async fn main() -> io::Result<()>  {
    let chat_room = Arc::new(ChatRoom::new());
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    loop {
        // Accept a new connection
        let (stream, _addr) = listener.accept().await?;
        let chat_room = Arc::clone(&chat_room);

        // Spawn a new task to handle the connection concurrently
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream,chat_room).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, chat_room: Arc<ChatRoom>)-> io::Result<()>  {

    // problem_0::echo(stream).await?;
    // problem_1::json_req_handler(stream).await?;
    // problem_2::handle_financial_requests(stream).await?;
    problem_3::handler::chat_handler(stream, chat_room).await?;
    Ok(())

}
