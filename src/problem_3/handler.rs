use std::sync::Arc;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::helpers::read_to_line_feed;
use crate::problem_3::room::ChatRoom;
/*
messages terminated by '\n'
must accept >= 1000 char messages

on connect ask for name
    resp is stored as user name >= 1 char
        allow at least 16 char len
        if name is < 1 char send error msg and disconnect
    user is joined to room
    send room participants to user starting with '*'
    broadcast join message starting with '*'

on disconnect
    broadcast leave message starting with '*'

on chat msg
    broadcast chat msg to all joined users starting with '[username] '
    chat msg must accept >= 1000 char
 */

pub async fn chat_handler(mut stream: TcpStream, store: Arc<ChatRoom>) -> io::Result<()> {
    let message = read_to_line_feed(&mut stream).await?;

    stream.shutdown().await?;
    Ok(())
}
