use io::types::Var;
use crate::handlers::Handler;
use io::connection::{Connection, ConnectionState};
use protocol::packets::login as login_packets;
use login_packets::serverbound::LoginStart;
use login_packets::clientbound::{ Packet as ClientboundPacket, LoginSuccess };
use protocol::packets::play as play_packets;
use play_packets::clientbound::{ Packet as PlayPacket, JoinGame, PlayerPositionAndLook, Dimension, Gamemode };
use failure::Fallible;

impl Handler for LoginStart {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) -> Fallible<()> {
        // TODO: Encryption packet
        // TODO: Compression packet
        // TODO: Remove uuid and username constants
        let packet = ClientboundPacket::LoginSuccess(LoginSuccess {
            uuid: "b8a85fff-60db-45b3-9a97-bbd88f3c6418".to_string().into(),
            username: "LightDiscord".to_string().into()
        });

        connection.send(&packet)?;
        connection.state = ConnectionState::Play;

        let packet = PlayPacket::JoinGame(JoinGame {
            entity_id: 0,
            gamemode: Gamemode::Spectator,
            dimension: Dimension::Overworld,
            hashed_seed: 0xfff00fff,
            max_player: 5,
            level_type: "default".to_string(),
            view_distance: Var(3),
            reduced_debug_info: false,
            enable_respawn_screen: true
        });

        //let packet = PlayPacket::SpawnPosition(SpawnPosition {
        //    location: Position {
        //        x: 50,
        //        y: 100,
        //        z: 50
        //    }
        //});

        connection.send(&packet)?;

        let packet = PlayPacket::PlayerPositionAndLook(PlayerPositionAndLook {
            x: 0.0,
            y: 100.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
            teleport_id: Var(0)
        });

        connection.send(&packet)?;

        Ok(())
    }
}

