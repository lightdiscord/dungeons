use crate::handlers::Handler;
use io::connection::Connection;
use protocol::packets::play as play_packets;
use play_packets::serverbound::ClientSettings;
use failure::Fallible;

impl Handler for ClientSettings {
    type Context = Connection;

    fn handle(&mut self, _connection: &mut Self::Context) -> Fallible<()> {
        Ok(())
    }
}

