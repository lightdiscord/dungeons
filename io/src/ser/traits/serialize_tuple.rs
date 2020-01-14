use serde::ser::{SerializeTuple, Serialize};
use super::Serializer;
use crate::Error;

impl SerializeTuple for &'_ mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

