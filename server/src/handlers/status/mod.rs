mod ping;
mod request;

use super::Handler;
use io::connection::Connection;
use protocol::packets::status as status_packets;
use status_packets::serverbound::Packet;
use failure::Fallible;

impl Handler for Packet {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) -> Fallible<()> {
        match self {
            Packet::Request(packet) => packet.handle(connection),
            Packet::Ping(packet) => packet.handle(connection)
        }
    }
}
