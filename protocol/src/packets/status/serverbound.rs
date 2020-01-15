use serde::Deserialize;
use io::packets;

#[derive(Debug, Deserialize)]
pub struct Request;

packets! {
    serverbound {
        0x00 => Request
    }
}
