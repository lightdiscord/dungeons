use io::connection::Connection;
use protocol::packets::handshaking::serverbound::{Packet as HandshakingPacket, Handshake};
use super::Handler;
use failure::Fallible;

impl Handler for HandshakingPacket {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) -> Fallible<()> {
        match self {
            HandshakingPacket::Handshake(packet) => packet.handle(connection)
        }
    }
}

impl Handler for Handshake {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) -> Fallible<()> {
        connection.state = self.next_state.into();
        Ok(())
    }
}
