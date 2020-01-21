use serde::ser::{self, Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct MaxedString<const M: usize>(String);

impl<const M: usize> From<String> for MaxedString<M> {
    fn from(src: String) -> Self {
        MaxedString(src)
    }
}

struct MaxedVisitor<const M: usize>;

impl<'de, const M: usize> Visitor<'de> for MaxedVisitor<M> {
    type Value = MaxedString<M>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string with a max length of {}", M)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        if value.as_bytes().len() > M {
            return Err(de::Error::invalid_length(value.len(), &self))
        }

        // TODO: Use Cow to avoid to_string
        Ok(MaxedString(value.to_string()))
    }
}

impl<'de, const M: usize> Deserialize<'de> for MaxedString<M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(MaxedVisitor::<M>)
    }
}

impl<'de, const M: usize> Serialize for MaxedString<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if self.0.as_bytes().len() > M {
            return Err(ser::Error::custom("maxed-string exceed max value"))
        }

        serializer.serialize_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Deserializer, Serializer};
    use bytes::Bytes;

    type Target = MaxedString<5>;

    const OK_BYTES: &'static [u8] = &[4, b't', b'e', b's', b't'];
    const ERR_BYTES: &'static [u8] = &[7, b't', b'o', b'o', b'l', b'o', b'n', b'g'];

    #[test]
    fn test_deserialize_maxed_string() {
        let ok: Result<Target, _> = Deserializer::from(Bytes::from(OK_BYTES)).deserialize();
        let err: Result<Target, _> = Deserializer::from(Bytes::from(ERR_BYTES)).deserialize();

        assert!(ok.is_ok());
        assert_eq!(ok.unwrap().0, "test".to_string());
        assert!(err.is_err());
    }

    #[test]
    fn test_serialize_maxed_string() {
        let mut serializer = Serializer::default();
        assert!(serializer.serialize(&MaxedString::<5>("test".to_string())).is_ok());
        assert_eq!(serializer.as_ref(), OK_BYTES);

        let mut serializer = Serializer::default();
        assert!(serializer.serialize(&MaxedString::<5>("toolong".to_string())).is_err());
    }
}
