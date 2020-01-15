use serde::Deserialize;
use io::packets;

#[derive(Debug, Deserialize)]
pub struct Request;

#[derive(Debug, Deserialize)]
pub struct Ping {
    pub payload: u64
}

packets! {
    serverbound {
        0x00 => Request,
        0x01 => Ping
    }
}
