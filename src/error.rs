use std::{error, fmt, result};

#[derive(Debug, Clone)]
pub enum Error {
    CouldNotParse(String),
    Custom(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CouldNotParse(string) => write!(f, "Could not parse: {}", string),
            Error::Custom(string) => write!(f, "Something went wrong: {}", string),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
