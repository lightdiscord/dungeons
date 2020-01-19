use serde::Serialize;
use bytes::{Bytes, BytesMut, Buf};
use crate::error::Result;

pub use super::Serializer;

mod serializer;
mod serialize_seq;
mod serialize_tuple;
mod serialize_tuple_struct;
mod serialize_tuple_variant;
mod serialize_struct;
mod serialize_struct_variant;
mod serialize_map;

impl Serializer {
    pub fn serialize<T>(&mut self, item: &T) -> Result<()>
    where
        T: Serialize
    {
        item.serialize(self)
    }
}

impl AsRef<BytesMut> for Serializer {
    fn as_ref(&self) -> &BytesMut {
        &self.0
    }
}

impl Into<Bytes> for Serializer {
    fn into(mut self) -> Bytes {
        self.0.to_bytes()
    }
}
