use io::packets;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Lol;

packets! {
    serverbound {
        0x00 => Lol
    }
}
