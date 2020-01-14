use serde::ser::{SerializeStruct, Serialize, SerializeSeq};
use super::Serializer;
use crate::Error;

impl SerializeStruct for &'_ mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<(), Self::Error> {
        SerializeSeq::end(self)
    }
}

