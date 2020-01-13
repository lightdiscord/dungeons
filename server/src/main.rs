#![feature(try_trait)]

use protocol::packets::handshaking::serverbound::Packet;

use std::io;

use tokio::stream::StreamExt;
use tokio::net::{TcpStream, TcpListener};
use tokio_util::codec::Decoder;

use ::io::MyDeserializer;

const ADDRESS: &'static str = "127.0.0.1:25565";

use ::io::codec::sized::SizedCodec;
use ::io::error::Error as MyError;

async fn process_stream(stream: TcpStream) -> Result<(), MyError> {
    let mut frames = SizedCodec::default().framed(stream);

    while let Some(frame) = frames.next().await {
        let packet: Packet = MyDeserializer::from(frame?).deserialize().unwrap();

        println!("received a packet: {:?}", packet);
    }

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
