use bytes::BytesMut;

mod traits;

#[derive(Default)]
pub struct Serializer(BytesMut);

#[cfg(test)]
pub mod tests {
    use serde::Serialize;
    use crate::types::Var;
    use crate::Error;
    use super::Serializer;

    #[test]
    fn test_structure_deserialization() -> Result<(), Error> {
        #[derive(Debug, Serialize, PartialEq)]
        struct Structure {
            var_int: Var<i32>,
            string: String,
            bit: u8,
            another_bit: u8
        }

        const BYTES: &'static [u8] = &[0xff, 0x01, 0x04, b't', b'e', b's', b't', 0x42, 0x00];

        let structure = Structure {
            var_int: Var(255),
            string: "test".to_string(),
            bit: 0x42,
            another_bit: 0x00
        };

        let mut serialized = Serializer::default();
        let _ = serialized.serialize(&structure)?;

        assert_eq!(BYTES, serialized.as_ref());

        Ok(())
    }
}
