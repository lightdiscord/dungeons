#![feature(try_trait)]

use protocol::packets::handshaking::serverbound::{Packet as HandshakingPacket, NextState};
use protocol::packets::status::serverbound::Packet as StatusPacket;
use futures::SinkExt;

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

use tokio_util::codec::Framed;

async fn process_stream(stream: TcpStream) -> Result<(), MyError> {
    let mut frames = Framed::new(stream, SizedCodec::default());

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
                        use protocol::packets::status::clientbound as status_clientbound;
                        let response = status_clientbound::Packet::Response(status_clientbound::Response {
                            json_response: status_clientbound::JsonResponse {
                                version: status_clientbound::JsonResponseVersion {
                                    name: "1.15.1".to_string(),
                                    protocol: 575
                                },
                                description: status_clientbound::JsonResponseDescription {
                                    text: "Ptdr ça marche enfin".to_string()
                                },
                                players: status_clientbound::JsonResponsePlayers {
                                    max: 5,
                                    online: 0,
                                    sample: Vec::new()
                                },
                                favicon: String::new()
                            }
                        });

                        println!("responseA");

                        use ::io::Serializer;
                        let mut serializer = Serializer::default();
                        println!("responseB");
                        serializer.serialize(&response)?;
                        println!("responseC");
                        frames.send(serializer.into()).await?;
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
