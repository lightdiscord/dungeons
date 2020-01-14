#![feature(try_trait)]

pub mod types;
pub mod error;
pub mod codec;

pub mod de;
pub mod ser;

pub use de::Deserializer;
pub use ser::Serializer;

mod macros;

use crate::error::Error;

