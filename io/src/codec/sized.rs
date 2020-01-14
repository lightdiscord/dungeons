use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, Bytes, Buf};
use std::io::Error as IoError;

use crate::types::var::Var;
use crate::Deserializer;
use crate::Error;

enum DecodeState {
    Head,
    Body(usize)
}

impl Default for DecodeState {
    fn default() -> Self { DecodeState::Head }
}

#[derive(Default)]
pub struct SizedCodec {
    state: DecodeState
}

impl SizedCodec {
    fn decode_head(&mut self, src: &mut BytesMut) -> Result<Option<usize>, Error> {
        let header_length = match Var::<i32>::count(&src[..])? {
            Some(n) => n,
            None => return Ok(None)
        };

        let packet_length: Var<i32> = Deserializer::from(src.split_to(header_length).to_bytes()).deserialize()?;
        let packet_length = *packet_length as usize;

        src.reserve(packet_length);
        Ok(Some(packet_length))
    }
}

impl Decoder for SizedCodec {
    type Item = Bytes;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let length = match self.state {
            DecodeState::Head => match self.decode_head(src)? {
                Some(n) => {
                    self.state = DecodeState::Body(n);
                    n
                },
                None => return Ok(None)
            },
            DecodeState::Body(n) => n
        };

        if src.len() < length {
            return Ok(None)
        }

        let bytes = src.split_to(length).to_bytes();

        self.state = DecodeState::Head;
        Ok(Some(bytes))
    }
}

impl Encoder for SizedCodec {
    type Item = Bytes;
    type Error = IoError;

    fn encode(&mut self, _item: Self::Item, _dest: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
