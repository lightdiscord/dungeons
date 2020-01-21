use super::Handler;
use io::connection::Connection;
use protocol::packets::play as play_packets;
use play_packets::serverbound::Packet;
use failure::Fallible;

impl Handler for Packet {
    type Context = Connection;

    fn handle(&mut self, _connection: &mut Self::Context) -> Fallible<()> {
        unimplemented!()
    }
}
