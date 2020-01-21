mod login;
mod encryption;

use super::Handler;
use io::connection::Connection;
use protocol::packets::login as login_packets;
use login_packets::serverbound::Packet;
use failure::Fallible;

impl Handler for Packet {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) -> Fallible<()> {
        match self {
            Packet::LoginStart(packet) => packet.handle(connection),
            Packet::EncryptionResponse(packet) => packet.handle(connection)
        }
    }
}
