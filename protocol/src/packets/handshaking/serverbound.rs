use serde::Deserialize;
use std::convert::TryFrom;
use io::derive::var::Var;
use io::packets;

use crate::error::Error;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "Var<i32>")]
pub enum NextState {
    Status,
    Login
}

impl TryFrom<Var<i32>> for NextState {
    type Error = Error;

    fn try_from(src: Var<i32>) -> Result<Self, Self::Error> {
        match *src {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            _ => Err(Error::InvalidNextState)
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Handshake {
    #[serde(with = "::io::derive::var::varint")]
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState
}

packets! {
    0x00 => Handshake
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::MyDeserializer;
    use bytes::Bytes;

    const PACKET: &'static [u8] = &[0, 191, 4, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 1];

    fn deserializer(skip: usize) -> MyDeserializer {
        MyDeserializer::from(Bytes::from(&PACKET[skip..]))
    }

    fn expected() -> Handshake {
        Handshake {
            protocol_version: 575,
            server_address: "127.0.0.1".to_string(),
            server_port: 25565,
            next_state: NextState::Status
        }
    }

    #[test]
    fn test_handshake_deserialization() {
        let deserialized: Handshake = deserializer(1).deserialize().unwrap();

        assert_eq!(deserialized, expected());
    }

    #[test]
    fn test_packet_deserialization() {
        let deserialized: Packet = deserializer(0).deserialize().unwrap();

        assert_eq!(deserialized, Packet::Handshake(expected()));
    }
}
