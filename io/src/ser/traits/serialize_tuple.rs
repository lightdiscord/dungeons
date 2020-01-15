use serde::ser::{SerializeTuple, Serialize};
use super::Serializer;
use crate::Error;

impl SerializeTuple for &'_ mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, item: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        self.serialize(&item)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

