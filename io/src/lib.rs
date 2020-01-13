#![feature(try_trait)]

use serde::de::{self, Visitor, SeqAccess, DeserializeSeed};
use bytes::{Bytes, Buf};

pub mod derive;
pub mod error;
pub mod codec;

mod macros;

use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct MyDeserializer {
    pub input: Bytes
}

impl From<Bytes> for MyDeserializer {
    fn from(src: Bytes) -> Self {
        MyDeserializer { input: src }
    }
}

use serde::Deserialize;

impl<'a> MyDeserializer {
    pub fn deserialize<T: Deserialize<'a>>(&'a mut self) -> Result<T> {
        T::deserialize(self)
    }
}

macro_rules! deserialize_unimplemented {
    ($($fn:ident),*) => {
        $(
            fn $fn<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
                unimplemented!()
            }
         )*
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut MyDeserializer {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }


    deserialize_unimplemented! {
        deserialize_bool,
        deserialize_byte_buf,
        deserialize_char,
        deserialize_f32,
        deserialize_f64,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i8,
        deserialize_identifier,
        deserialize_ignored_any,
        deserialize_map,
        deserialize_option,
        deserialize_str,
        deserialize_u32,
        deserialize_u64,
        deserialize_bytes,
        deserialize_unit
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let length = derive::var::varint::deserialize(&mut *self)? as usize;
        let string = String::from_utf8(self.input.slice(..length).to_vec())?;
        self.input.advance(length);
        visitor.visit_string(string)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.input.get_u8())
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(self.input.get_u16())
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        struct Access<'a> {
            deserializer: &'a mut MyDeserializer
        }

        impl <'de, 'a> SeqAccess<'de> for Access<'a> {
            type Error = Error;

            fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
                Ok(Some(T::deserialize(seed, &mut *self.deserializer)?))
            }
        }
        
        visitor.visit_seq(Access { deserializer: self })
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(self, _: &'static str, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _: &'static str, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        struct Access<'a> {
            deserializer: &'a mut MyDeserializer,
            len: usize
        }

        impl<'de, 'a> SeqAccess<'de> for Access<'a> {
            type Error = Error;

            fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
                if self.len == 0 {
                    Ok(None)
                } else {
                    self.len -= 1;
                    let value = DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access { deserializer: self, len })
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(self, _: &'static str, _: usize, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_struct<V: Visitor<'de>>(self, _: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(self, _: &'static str, _: &'static [&'static str], _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }
}
