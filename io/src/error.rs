use std::fmt::{self, Display};
use std::error::Error as StdError;
use serde::{de, ser};

use std::io::Error as IoError;
use std::option::NoneError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    Custom(String),

    VarIntTooBig,

    NoneError,
    IoError(IoError),
    Utf8Error(FromUtf8Error)
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("zeezaazeaez")
    }
}

impl de::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        Error::Custom(message.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        Error::Custom(message.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::Utf8Error(error)
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NoneError
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::IoError(error)
    }
}

impl StdError for Error { }
