use serde::{Deserializer, Serializer};
use serde::de::{self, Visitor, SeqAccess};
use std::fmt;
use std::ops::Deref;
use std::marker::PhantomData;
use crate::Error;

pub struct Var<T>(T);

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

        Err(de::Error::custom(Error::VarIntTooBig))
    }
}

struct VarVisitor<T>(PhantomData<T>);

pub mod varint {
    use super::*;
    use serde::de::Deserialize;

    impl <'de> Visitor<'de> for VarVisitor<i32> {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("varint")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut idx = 0;
            let mut result: i32 = 0;

            loop {
                let byte: u8 = seq.next_element()?
                    .ok_or(Error::NoneError)
                    .map_err(de::Error::custom)?;

                let value = byte & 0b01111111;
                result |= (value as i32) << (7 * idx);
                idx += 1;

                if idx > 5 {
                    return Err(de::Error::custom(Error::VarIntTooBig));
                }
                if (byte & 0b10000000) == 0 { break; }
            }

            Ok(result)
        }
    }


    impl<'de> Deserialize<'de> for Var<i32> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserialize(deserializer).map(Var)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i32, D::Error> {
        deserializer.deserialize_seq(VarVisitor(PhantomData::<i32>))
    }

    pub fn serializer<S: Serializer>(_value: i32, _serializer: S) -> Result<S::Ok, S::Error> {
        unimplemented!()
    }
}
