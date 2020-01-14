use serde::Deserialize;
use io::packets;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Request;

packets! {
    0x00 => Request
}
