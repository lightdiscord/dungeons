use failure::{Fail, Error as FailureError, format_err};
use std::fmt::{self, Display};
use std::error::Error as StdError;
use std::str::Utf8Error;
use serde::{de, ser};

#[derive(Debug, Fail)]
pub enum PacketError {
    #[fail(display = "unknown packet id")]
    UnknownPacket,

    #[fail(display = "option with some variant expected but got none")]
    NoneError
}

#[derive(Debug)]
pub struct Error(FailureError);

impl Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl de::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        Error(format_err!("deserialization error: {}", message.to_string()))
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        Error(format_err!("serialization error: {}", message.to_string()))
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error(error.into())
    }
}

impl StdError for Error { }

pub type Result<T> = std::result::Result<T, Error>;
