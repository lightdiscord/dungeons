mod handshaking;
mod status;

use bytes::Bytes;
use io::Deserializer;
use io::connection::{Connection, ConnectionState};
use protocol::packets::handshaking::serverbound::Packet as HandshakingPacket;
use protocol::packets::status::serverbound::Packet as StatusPacket;

pub trait Handler {
    type Context;

    fn handle(&mut self, context: &mut Self::Context);
}

impl Handler for Connection {
    type Context = Bytes;

    fn handle(&mut self, bytes: &mut Self::Context) {
        match self.state {
            ConnectionState::Handshaking => {
                let mut packet: HandshakingPacket = Deserializer::from(bytes.clone()).deserialize().unwrap();
                packet.handle(self)
            },

            ConnectionState::Status => {
                let mut packet: StatusPacket = Deserializer::from(bytes.clone()).deserialize().unwrap();
                packet.handle(self)
            },

            ConnectionState::Login => {
                unimplemented!()
            }
        }
    }
}
