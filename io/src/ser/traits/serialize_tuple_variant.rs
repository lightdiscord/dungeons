use serde::ser::{SerializeTupleVariant, Serialize};
use super::Serializer;
use crate::Error;

impl SerializeTupleVariant for &'_ mut Serializer {
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

