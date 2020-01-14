use crate::Error;
use crate::types::Var;
use serde::de::{self, Visitor, DeserializeSeed};
use bytes::Buf;

use super::Deserializer;

type Result<T> = std::result::Result<T, Error>;

type StaticStr = &'static str;

macro_rules! deserialize_unimplemented {
    ($($fn:ident$(($($type:ty),*))*),*) => {
        $(
            fn $fn<V>(self, $($(_: $type),*,)* _: V) -> Result<V::Value>
            where
                V: Visitor<'de>
            {
                unimplemented!(stringify!($fn))
            }
        )*
    };
}

macro_rules! deserialize_simple {
    ($(($deserialize:ident, $visit:ident, $get:ident)),*) => {
        $(
            fn $deserialize<V>(self, visitor: V) -> Result<V::Value>
            where
                V: Visitor<'de>
            {
                visitor.$visit((self.0).$get())
            }
        )*
    }
}

struct SeqAccess<'a>(&'a mut Deserializer);

impl <'de> de::SeqAccess<'de> for SeqAccess<'_> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        Ok(Some(T::deserialize(seed, &mut *self.0)?))
    }
}

impl<'de> de::Deserializer<'de> for &'_ mut Deserializer {
    type Error = Error;

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        let length = *(&mut *self).deserialize::<Var<i32>>()? as usize;
        let string = String::from_utf8(self.0.split_to(length).to_vec())?;
        visitor.visit_string(string)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        visitor.visit_seq(SeqAccess(self))
    }

    fn deserialize_unit_struct<V>(self, _: StaticStr, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple<V>(self, _: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_struct<V>(self, _: StaticStr, _: &'static [StaticStr], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    deserialize_unimplemented! {
        deserialize_any,
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
        deserialize_unit,
        deserialize_newtype_struct(StaticStr),
        deserialize_tuple_struct(StaticStr, usize),
        deserialize_enum(StaticStr, &'static [StaticStr])
    }

    deserialize_simple! {
        (deserialize_u8, visit_u8, get_u8),
        (deserialize_u16, visit_u16, get_u16)
    }
}
