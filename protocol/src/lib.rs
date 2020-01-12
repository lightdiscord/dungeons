use std::ops::Deref;

mod packets;

#[derive(Debug)]
pub enum Packet {
    Handshaking(HandshakingPacket),
}

#[derive(Debug)]
pub enum NextState {
    Status,
    Login
}

#[derive(Debug, Clone, Copy)]
pub struct VarInt(pub i32);

#[derive(Debug)]
pub enum HandshakingPacket {
    Handshake {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: NextState
    }
}

impl Deref for VarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
