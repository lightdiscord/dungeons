use std::marker::PhantomData;
use std::fmt;
use serde::{ser, de};
use serde_json;

struct JsonVisitor<T>(PhantomData<T>);

impl<'de, T> de::Visitor<'de> for JsonVisitor<T>
where 
    T: de::DeserializeOwned
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("json string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        serde_json::from_str(value).map_err(de::Error::custom)
    }
}

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ser::Serialize,
    S: ser::Serializer
{
    let string = serde_json::to_string(value).map_err(ser::Error::custom)?;
    serializer.serialize_str(&string)
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::DeserializeOwned
{
    deserializer.deserialize_str(JsonVisitor(PhantomData::<T>))
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use crate::{Serializer, Deserializer, Error};
    use bytes::Bytes;
    use failure::Fallible;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Data {
        string: String,
        b: bool,
        n: usize
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Structure {
        #[serde(with = "super")]
        data: Data
    }

    fn expected_structure() -> Structure {
        Structure {
            data: Data {
                string: "test".to_string(),
                b: true,
                n: 42
            }
        }
    }

    #[test]
    fn test_serialize_json_string() -> Result<(), Error> {
        let mut serializer = Serializer::default();
        let _ = serializer.serialize(&expected_structure())?;

        let expected_json: &[u8] = br#"{"string":"test","b":true,"n":42}"#;
        let expected_bytes = [&[33], expected_json].concat();

        assert_eq!(
            serializer.as_ref(),
            &expected_bytes
        );

        Ok(())
    }

    #[test]
    fn test_deserialize_json_string() -> Fallible<()> {
        let json: &[u8] = br#"{"string":"test","b":true,"n":42}"#;
        let bytes = [&[33], json].concat();
        let bytes = Bytes::from(bytes);

        let structure: Structure = Deserializer::from(bytes).deserialize()?;

        assert_eq!(
            structure,
            expected_structure()
        );

        Ok(())
    }
}
