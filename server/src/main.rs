#![feature(try_trait)]

use protocol::packets::handshaking::serverbound::{Packet as HandshakingPacket, NextState};
use protocol::packets::status::serverbound::Packet as StatusPacket;

use std::io;

use tokio::stream::StreamExt;
use tokio::net::{TcpStream, TcpListener};
use tokio_util::codec::Decoder;

use ::io::Deserializer;

const ADDRESS: &'static str = "127.0.0.1:25565";

use ::io::codec::sized::SizedCodec;
use ::io::error::Error as MyError;

enum State {
    Handshaking,
    Login,
    Status,
    Play
}

async fn process_stream(stream: TcpStream) -> Result<(), MyError> {
    let mut frames = SizedCodec::default().framed(stream);

    let mut state = State::Handshaking;

    while let Some(frame) = frames.next().await {
        println!("received a packet");

        match state {
            State::Handshaking => {
                let packet: HandshakingPacket = Deserializer::from(frame?).deserialize().unwrap();

                match packet {
                    HandshakingPacket::Handshake(packet) => {
                        state = match packet.next_state {
                            NextState::Login => State::Login,
                            NextState::Status => State::Status
                        }
                    }
                }
            },
            State::Status => {
                let packet: StatusPacket = Deserializer::from(frame?).deserialize().unwrap();

                match packet {
                    StatusPacket::Request(_) => {
                    }
                }

                println!("packet = {:?}", packet);
            },
            _ => unimplemented!()
        }
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
