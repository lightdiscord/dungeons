use serde::Serialize;
use bytes::BytesMut;
use crate::Error;

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
    pub fn serialize<T>(&mut self, item: &T) -> Result<(), Error>
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
