use std;
use std::fmt::{self, Display};
use std::io;
use std::io::ErrorKind;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Error(io::Error),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Error(io::Error::new(ErrorKind::Other, msg.to_string()))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Error(io::Error::new(ErrorKind::Other, msg.to_string()))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Error(err) => formatter.write_str(&err.to_string()),
        }
    }
}

impl std::error::Error for Error {}
