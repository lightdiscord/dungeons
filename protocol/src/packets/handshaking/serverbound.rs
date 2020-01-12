use serde::Deserialize;
use std::convert::TryFrom;
use io::derive::var::Var;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "Var<i32>")]
pub enum NextState {
    Status,
    Login
}

impl TryFrom<Var<i32>> for NextState {
    // TODO: Error type
    type Error = usize;

    fn try_from(src: Var<i32>) -> Result<Self, Self::Error> {
        match *src {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            _ => Err(0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use io::MyDeserializer;
    use bytes::BytesMut;

    #[test]
    fn test_handshake_deserialization() {
        const MESSAGE: &'static [u8] = &[191, 4, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 1];

        let mut message = BytesMut::from(MESSAGE);
        let mut deserializer = MyDeserializer { input: &mut message };
        let deserialized: Handshake = Handshake::deserialize(&mut deserializer).unwrap();

        let expected = Handshake {
            protocol_version: 575,
            server_address: "127.0.0.1".to_string(),
            server_port: 25565,
            next_state: NextState::Status
        };

        assert_eq!(deserialized, expected);
    }
}
