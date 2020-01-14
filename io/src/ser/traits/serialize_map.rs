use serde::ser::{SerializeMap, Serialize};
use super::Serializer;
use crate::Error;

impl SerializeMap for &'_ mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
