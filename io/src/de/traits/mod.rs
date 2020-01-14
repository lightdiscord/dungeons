use serde::Deserialize;
use bytes::Bytes;
use crate::Error;

pub use super::Deserializer;

mod deserializer;

impl From<Bytes> for Deserializer {
    fn from(bytes: Bytes) -> Self { Deserializer(bytes) }
}

impl<'a> Deserializer {
    pub fn deserialize<T>(&mut self) -> Result<T, Error>
    where
        T: Deserialize<'a>
    {
        T::deserialize(self)
    }
}

