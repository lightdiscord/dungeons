use serde::Deserialize;
use io::serverbound_packets;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Request;

serverbound_packets! {
    0x00 => Request
}
