use crate::Error;
use serde::ser;
use super::Serializer;
use bytes::BufMut;
use crate::types::Var;

type Result<T> = std::result::Result<T, Error>;

macro_rules! serialize_unimplemented {
    ($($return:ty => [$($fn:ident$(($($type:ty),*))*),*]),*) => {
        $($(
            fn $fn(self $(,$(_: $type),*)*) -> Result<$return> {
                unimplemented!(stringify!($fn))
            }
         )*)*
    }
}

impl ser::Serializer for &'_ mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.0.put_u8(value);

        Ok(())
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.serialize(&Var(value.len() as i32))?;
        self.0.extend_from_slice(value.as_bytes());

        Ok(())
    }

    fn serialize_struct(self, _: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }


    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    serialize_unimplemented! {
        Self::Ok => [
            serialize_bool(bool),
            serialize_char(char),
            serialize_f32(f32),
            serialize_f64(f64),
            serialize_i16(i16),
            serialize_u16(u16),
            serialize_i32(i32),
            serialize_i64(i64),
            serialize_i8(i8),
            serialize_unit_struct(&str),
            serialize_u32(u32),
            serialize_u64(u64),
            serialize_bytes(&[u8]),
            serialize_unit,
            serialize_none,
            serialize_unit_variant(&'static str, u32, &'static str)
        ],

        Self => [
            serialize_map(Option<usize>),
            serialize_tuple_struct(&'static str, usize),
            serialize_struct_variant(&'static str, u32, &'static str, usize),
            serialize_tuple_variant(&'static str, u32, &'static str, usize)
        ]
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize
    {
        unimplemented!()
    }
}
