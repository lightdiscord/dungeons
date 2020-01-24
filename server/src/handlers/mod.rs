mod handshaking;
mod status;
mod login;
mod play;

use bytes::Bytes;
use io::Deserializer;
use io::connection::{Connection, ConnectionState};
use protocol::packets::handshaking::serverbound::Packet as HandshakingPacket;
use protocol::packets::status::serverbound::Packet as StatusPacket;
use protocol::packets::login::serverbound::Packet as LoginPacket;
use protocol::packets::play::serverbound::Packet as PlayPacket;
use failure::Fallible;
use log::trace;

pub trait Handler {
    type Context;

    fn handle(&mut self, context: &mut Self::Context) -> Fallible<()>;
}

impl Handler for Connection {
    type Context = Bytes;

    fn handle(&mut self, bytes: &mut Self::Context) -> Fallible<()> {
        trace!("during state {:?}, received bytes: {:?}", self.state, bytes.as_ref());

        match self.state {
            ConnectionState::Handshaking => {
                let mut packet: HandshakingPacket = Deserializer::from(bytes.clone()).deserialize()?;
                packet.handle(self)
            },

            ConnectionState::Status => {
                let mut packet: StatusPacket = Deserializer::from(bytes.clone()).deserialize()?;
                packet.handle(self)
            },

            ConnectionState::Login => {
                let mut packet: LoginPacket = Deserializer::from(bytes.clone()).deserialize()?;
                packet.handle(self)
            },

            ConnectionState::Play => {
                let mut packet: PlayPacket = Deserializer::from(bytes.clone()).deserialize()?;
                packet.handle(self)
            }
        }
    }
}
