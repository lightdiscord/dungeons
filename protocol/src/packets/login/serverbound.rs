use serde::Deserialize;
use io::packets;
use io::types::MaxedString;

#[derive(Debug, Deserialize)]
pub struct LoginStart {
    name: MaxedString<16>
}

#[derive(Debug, Deserialize)]
pub struct EncryptionResponse {
    shared_secret: Vec<u8>,
    verify_token: Vec<u8>
}

packets! {
    serverbound {
        0x00 => LoginStart,
        0x01 => EncryptionResponse
    }
}
