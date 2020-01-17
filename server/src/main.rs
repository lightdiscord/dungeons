#![feature(try_trait, async_closure)]

mod handlers;

use std::io;

use futures::future;
use futures::stream::StreamExt;
use tokio::net::{TcpStream, TcpListener};

const ADDRESS: &'static str = "127.0.0.1:25565";

use ::io::codec::sized::SizedCodec;
use ::io::error::Error as MyError;

#[allow(dead_code)]
enum State {
    Handshaking,
    Login,
    Status,
    Play
}

use tokio::sync::mpsc;

use crate::handlers::Handler;
use tokio_util::codec::Framed;
use ::io::Connection;

async fn process_stream(stream: TcpStream) -> Result<(), MyError> {
    let (tx, rx) = mpsc::unbounded_channel();

    let mut connection = Connection::new(tx);
    let (sink, stream) = Framed::new(stream, SizedCodec::default()).split();

    let to_user = rx
        .map(|bytes| Ok(bytes))
        .forward(sink);

    let from_user = stream
        .for_each(|packet| future::ready(connection.handle(&mut packet.unwrap())));

    let _ = future::join(to_user, from_user).await;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind(ADDRESS).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async { process_stream(socket).await });
    }
}
