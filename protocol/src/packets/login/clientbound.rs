use io::types::{Var, MaxedString};
use serde::Serialize;
use io::packets;

#[derive(Debug, Serialize)]
pub struct Disconnect {
    // TODO: Create a Chat type
    reason: String
}

#[derive(Debug, Serialize)]
pub struct EncryptionRequest {
    server_id: MaxedString<20>,
    public_key: Vec<u8>,
    verify_token: Vec<u8>
}

#[derive(Debug, Serialize)]
pub struct LoginSuccess {
    // TODO: Uuid type
    pub uuid: MaxedString<36>,
    pub username: MaxedString<16>
}

#[derive(Debug, Serialize)]
pub struct SetCompression {
    threshold: Var<i32>
}

packets! {
    clientbound {
        0x00 => Disconnect,
        0x01 => EncryptionRequest,
        0x02 => LoginSuccess,
        0x03 => SetCompression
    }
}
