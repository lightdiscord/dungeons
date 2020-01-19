use serde::de::{self, Visitor, SeqAccess};
use std::fmt;
use std::ops::Deref;
use std::marker::PhantomData;
use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Var<T>(pub T);

impl<T> Deref for Var<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl Var<i32> {
    pub(crate) fn count(bytes: &[u8]) -> Result<Option<usize>, Error> {
        for idx in 0..5 {
            let byte = match bytes.get(idx) {
                Some(byte) => byte,
                None => return Ok(None)
            };

            if (byte & 0b10000000) == 0 {
                return Ok(Some(idx + 1));
            }
        }

        Err(de::Error::invalid_length(6, &"varint"))
    }
}

pub struct VarVisitor<T>(PhantomData<T>);

pub mod varint {
    use super::*;
    use serde::de::Deserialize;

    impl<'de> Visitor<'de> for VarVisitor<i32> {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("varint")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error> 
        where
            S: SeqAccess<'de>,
        {
            let mut result: i32 = 0;

            for idx in 0..5 {
                let byte: u8 = seq.next_element()?.ok_or(de::Error::invalid_length(idx, &self))?;

                let value = byte & 0b01111111;
                result |= (value as i32) << (7 * idx);

                if (byte & 0b10000000) == 0 {
                    return Ok(result)
                }
            }

            Err(de::Error::invalid_length(6, &self))
        }
    }


    impl<'de> Deserialize<'de> for Var<i32> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>
        {
            deserialize(deserializer).map(Var)
        }
    }

    use serde::ser::{Serialize, Serializer, SerializeSeq};

    impl Serialize for Var<i32> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
        {
            let mut value = **self as u32;
            let mut sequence = serializer.serialize_seq(None)?;

            loop {
                let mut temp = (value & 0b01111111) as u8;
                value >>= 7;
                if value != 0 {
                    temp |= 0b10000000
                }
                sequence.serialize_element(&temp)?;
                if value == 0 { break; }
            }

            sequence.end()
        }
    }
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    VarVisitor<T>: Visitor<'de, Value = T>,
    D: de::Deserializer<'de>
{
    deserializer.deserialize_seq(VarVisitor(PhantomData::<T>))
}

use serde::ser::{Serializer, Serialize};

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    Var<T>: Serialize,
    T: Copy
{
    Var(*value).serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use failure::Fallible;
    use bytes::Bytes;
    use crate::{Deserializer, Serializer};

    const VARS: &'static [(Var<i32>, &'static [u8])] = &[
        (Var(0), &[0]),
        (Var(1), &[1]),
        (Var(2), &[2]),
        (Var(127), &[127]),
        (Var(128), &[128, 1]),
        (Var(255), &[255, 1]),
        (Var(2147483647), &[255, 255, 255, 255, 7]),
        (Var(-1), &[255, 255, 255, 255, 15]),
        (Var(-2147483648), &[128, 128, 128, 128, 8]),
    ];

    #[test]
    fn test_varint_deserialization() -> Fallible<()> {
        for &(value, bytes) in VARS {
            assert_eq!(
                value,
                Deserializer::from(Bytes::from(bytes)).deserialize()?
            );
        }

        Ok(())
    }

    #[test]
    fn test_varint_serialization() -> Result<(), Error> {
        for &(value, bytes) in VARS {
            let mut serializer = Serializer::default();
            let _ = serializer.serialize(&value)?;

            assert_eq!(
                serializer.as_ref(),
                bytes
            );
        }

        Ok(())
    }
}
