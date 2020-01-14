use serde::ser::{SerializeTupleStruct, Serialize};
use super::Serializer;
use crate::Error;

impl SerializeTupleStruct for &'_ mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

