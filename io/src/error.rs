use std::fmt::{self, Display};
use std::error::Error as StdError;
use serde::de;

use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    Custom(String),

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

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::Utf8Error(error)
    }
}

impl StdError for Error { }
