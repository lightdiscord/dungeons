use serde::Deserialize;
use std::convert::TryFrom;
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
    #[serde(with = "::io::types::var")]
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState
}

packets! {
    0x00 => Handshake
}
