#[macro_export]
macro_rules! serverbound_packets {
    (
        $(
            $id:expr => $packet:ident
        ),*
    ) => {
        use $crate::types::Var;
        use serde::de::{self, Deserializer, Visitor, SeqAccess};
        use std::fmt;

        #[derive(Debug, PartialEq)]
        pub enum Packet {
            $($packet($packet)),*
        }

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
                match *seq.next_element::<Var<i32>>()?.unwrap() {
                    $($id => Ok(Packet::$packet(seq.next_element()?.unwrap()))),*,
                    _ => Err(de::Error::custom("Unknown packet"))
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
}

#[macro_export]
macro_rules! clientbound_packets {
    (
        $(
            $id:expr => $packet:ident
        ),+
    ) => {
        use $crate::types::Var;
        use std::fmt;
        use serde::ser;

        #[derive(Debug)]
        pub enum Packet {
            $($packet($packet)),+
        }

        impl ser::Serialize for Packet {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer
            {
                use ser::SerializeTuple;

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
}
