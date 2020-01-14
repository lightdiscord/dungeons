use serde::ser::{SerializeStructVariant, Serialize};
use super::Serializer;
use crate::Error;

impl SerializeStructVariant for &'_ mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        unimplemented!()
    }

    fn end(self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

