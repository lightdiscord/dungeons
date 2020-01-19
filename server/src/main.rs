#![feature(try_trait, async_closure)]

mod handlers;

use crate::handlers::Handler;
use io::codec::sized::SizedCodec;
use io::Connection;
use futures::future::{self, TryFutureExt};
use futures::stream::{TryStreamExt, StreamExt};
use tokio::net::{TcpStream, TcpListener};
use tokio::sync::mpsc;
use tokio_util::codec::Framed;
use failure::Fallible;

const ADDRESS: &'static str = "127.0.0.1:25565";

async fn process_stream(stream: TcpStream) -> Fallible<()> {
    let (tx, rx) = mpsc::unbounded_channel();

    let mut connection = Connection::new(tx);
    let (sink, stream) = Framed::new(stream, SizedCodec::default()).split();

    let to_user = rx
        .map(|bytes| Ok(bytes))
        .forward(sink);

    let from_user = stream
        .try_for_each(|mut packet| future::ready(connection.handle(&mut packet)))
        .into_future();

    future::try_join(to_user, from_user).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Fallible<()> {
    let mut listener = TcpListener::bind(ADDRESS).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async { process_stream(socket).await });
    }
}
