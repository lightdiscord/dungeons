mod client_settings;

use super::Handler;
use io::connection::Connection;
use protocol::packets::play as play_packets;
use play_packets::serverbound::Packet;
use failure::Fallible;

impl Handler for Packet {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) -> Fallible<()> {
        match self {
            Packet::ClientSettings(packet) => packet.handle(connection),
            Packet::PluginMessage(_) => Ok(()),
            Packet::TeleportConfirm(_) => Ok(()),
            Packet::PlayerPositionAndLook(_) => Ok(())
        }
    }
}
