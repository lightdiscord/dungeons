#[macro_export]
macro_rules! packets {
    (
        $(
            $id:expr => $packet:ident
        ),*
    ) => {
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
