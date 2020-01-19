use bytes::Bytes;

mod traits;

pub struct Deserializer(Bytes);

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use crate::types::Var;
    use super::Deserializer;
    use bytes::Bytes;
    use crate::error::Result;

    #[test]
    fn test_structure_deserialization() -> Result<()> {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Structure {
            var_int: Var<i32>,
            string: String,
            bit: u8,
            another_bit: u8
        }

        const BYTES: &'static [u8] = &[0xff, 0x01, 0x04, b't', b'e', b's', b't', 0x42, 0x00];

        let expected = Structure {
            var_int: Var(255),
            string: "test".to_string(),
            bit: 0x42,
            another_bit: 0x00
        };

        let bytes = Bytes::from(BYTES);
        let deserialized: Structure = Deserializer::from(bytes).deserialize()?;

        assert_eq!(expected, deserialized);

        Ok(())
    }
}
