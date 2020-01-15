use serde::Serialize;
use io::clientbound_packets;

#[derive(Debug, Serialize)]
pub struct JsonResponseVersion {
    pub name: String,
    pub protocol: usize
}

#[derive(Debug, Serialize)]
pub struct JsonResponsePlayer {
    pub name: String,
    pub id: String
}

#[derive(Debug, Serialize)]
pub struct JsonResponsePlayers {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<JsonResponsePlayer>
}

#[derive(Debug, Serialize)]
pub struct JsonResponseDescription {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct JsonResponse {
    pub version: JsonResponseVersion,
    pub players: JsonResponsePlayers,
    pub description: JsonResponseDescription,
    pub favicon: String
}

#[derive(Debug, Serialize)]
pub struct Response {
    #[serde(with = "io::types::json")]
    pub json_response: JsonResponse
}

clientbound_packets! {
    0x00 => Response
}
