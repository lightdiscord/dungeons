use crate::handlers::Handler;
use io::connection::Connection;
use protocol::packets::status as status_packets;
use status_packets::serverbound::Request;
use status_packets::clientbound::{Packet as ClientboundPacket, Response};
use status_packets::clientbound::json_response::*;

impl Handler for Request {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) {
        let response = ClientboundPacket::Response(Response {
            json_response: JsonResponse {
                version: JsonResponseVersion {
                    name: "1.15.1".to_string(),
                    protocol: 575
                },
                description: JsonResponseDescription {
                    text: "Ptdr ça marche enfin".to_string()
                },
                players: JsonResponsePlayers {
                    max: 5,
                    online: 0,
                    sample: Vec::new()
                },
                favicon: String::new()
            }
        });

        connection.send(&response);
    }
}

