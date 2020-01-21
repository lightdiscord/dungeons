#![feature(try_trait, async_closure)]

mod handlers;

use crate::handlers::Handler;
use io::codec::sized::SizedCodec;
use futures::future;
use futures::stream::StreamExt;
use futures::sink::SinkExt;
use tokio::net::{TcpStream, TcpListener};
use tokio::sync::mpsc;
use tokio_util::codec::Framed;
use failure::Fallible;
use bytes::Bytes;
use tokio::sync::mpsc::UnboundedReceiver;
use futures::stream::{SplitSink, SplitStream};
use io::connection::{Connection, ConnectionEvent};
use log::{info, error};

const ADDRESS: &'static str = "127.0.0.1:25565";

async fn forward_to_sink(mut sink: SplitSink<Framed<TcpStream, SizedCodec>, Bytes>, receiver: &mut UnboundedReceiver<ConnectionEvent>) -> Fallible<()> {
    while let Some(event) = receiver.next().await {
        match event {
            ConnectionEvent::Message(message) => sink.send(message).await?,
            ConnectionEvent::Close => receiver.close()
        }
    }

    Ok(())
}

async fn handle_each_packet(mut stream: SplitStream<Framed<TcpStream, SizedCodec>>, mut connection: Connection) -> Fallible<()> {
    while let Some(message) = stream.next().await {
        connection.handle(&mut message?)?;
    }

    connection.close()
}

async fn process_stream(stream: TcpStream) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let peer_addr = stream.peer_addr().unwrap();

    info!("connection '{}' opened", peer_addr);

    let connection = Connection::new(tx);
    let (sink, stream) = Framed::new(stream, SizedCodec::default()).split();

    let to_client = forward_to_sink(sink, &mut rx);
    let from_client = handle_each_packet(stream, connection);

    match future::try_join(to_client, from_client).await {
        Ok(_) => info!("connection '{}' closed with success", peer_addr),
        Err(error) => error!("connection '{}' closed with an error ({})", peer_addr, error)
    }
}

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let mut listener = TcpListener::bind(ADDRESS).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async { process_stream(socket).await });
    }
}
