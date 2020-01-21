#[macro_export]
macro_rules! packets {
    (
        $(#[$enum_meta:meta])*
        common {
            $($packet:ident),+
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug)]
        pub enum Packet {
            $($packet($packet)),*
        }
    };

    (
        $(#[$enum_meta:meta])*
        serverbound {
            $($id:expr => $packet:ident),+
        }
    ) => {
        packets! {
            $(#[$enum_meta])*
            common { $($packet),+ }
        }

        mod __impl_packets_traits {
            use serde::de::{self, Visitor, SeqAccess, Deserialize, Deserializer};
            use $crate::types::Var;
            use $crate::error::PacketError;
            use super::Packet;
            use std::fmt;

            struct PacketVisitor;

            impl<'de> Visitor<'de> for PacketVisitor {
                type Value = Packet;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a minecraft packet id and data")
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>
                {
                    match *seq.next_element::<Var<i32>>()?.ok_or(de::Error::custom(PacketError::NoneError))? {
                        $($id => Ok(Packet::$packet(seq.next_element()?.ok_or(de::Error::custom(PacketError::NoneError))?))),+,
                        id => Err(de::Error::custom(PacketError::UnknownPacket(id)))
                    }
                }
            }

            impl<'de> Deserialize<'de> for Packet {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>
                {
                    deserializer.deserialize_tuple(2, PacketVisitor)
                }
            }
        }
    };

    (
        $(#[$enum_meta:meta])*
        clientbound {
            $($id:expr => $packet:ident),+
        }
    ) => {
        packets! {
            $(#[$enum_meta])*
            common { $($packet),+ }
        }

        mod __impl_packets_traits {
            use serde::ser::{Serialize, Serializer, SerializeTuple};
            use $crate::types::Var;
            use super::Packet;

            impl Serialize for Packet {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer
                {

                    let mut tuple = serializer.serialize_tuple(2)?;
                    match self {
                        $(Packet::$packet(packet) => {
                            tuple.serialize_element(&Var($id))?;
                            tuple.serialize_element(&packet)?;
                        }),+
                    }
                    tuple.end()
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use crate::{Deserializer, Serializer};
    use crate::types::Var;
    use failure::Fallible;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct PacketData {
        number: Var<i32>,
        another_number: Var<i32>,
        bit: u8
    }

    fn packet_data() -> PacketData {
        PacketData {
            number: Var(2),
            another_number: Var(255),
            bit: 42
        }
    }

    const PACKET_ID: u8 = 101;
    const EXPECTED_PACKET_BYTES: &'static [u8] = &[PACKET_ID, 2, 0xff, 0x01, 42];

    mod serverbound {
        use super::*;
        use bytes::Bytes;

        packets! {
            #[derive(PartialEq)]
            serverbound {
                101 => PacketData
            }
        }

        #[test]
        fn test_packet_deserialization() -> Fallible<()> {
            let packet: Packet = Deserializer::from(Bytes::from(EXPECTED_PACKET_BYTES)).deserialize()?;
            
            assert_eq!(packet, Packet::PacketData(packet_data()));

            Ok(())
        }
    }

    mod clientbound {
        use super::*;

        packets! {
            #[derive(PartialEq)]
            clientbound {
                101 => PacketData
            }
        }

        #[test]
        fn test_packet_deserialization() -> Fallible<()> {
            let mut serializer = Serializer::default();
            let _ = serializer.serialize(&Packet::PacketData(packet_data()))?;
            
            assert_eq!(serializer.as_ref(), EXPECTED_PACKET_BYTES);

            Ok(())
        }
    }
}
