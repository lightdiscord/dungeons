use serde::Deserialize;
use std::convert::TryFrom;
use io::packets;
use io::types::{Var, MaxedString};
use io::connection::ConnectionState;

use crate::error::Error;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
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

impl Into<ConnectionState> for NextState {
    fn into(self) -> ConnectionState {
        match self {
            NextState::Status => ConnectionState::Status,
            NextState::Login => ConnectionState::Login
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Handshake {
    #[serde(with = "::io::types::var")]
    pub protocol_version: i32,
    pub server_address: MaxedString<255>,
    pub server_port: u16,
    pub next_state: NextState
}

packets! {
    serverbound {
        0x00 => Handshake
    }
}
