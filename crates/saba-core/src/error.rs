use alloc::string::String;
use core::{fmt, num::ParseIntError};

#[derive(Debug, Clone)]
pub enum Error {
    InvalidScheme(String),
    InvalidPort(ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidScheme(url) => write!(f, "Support only http (url: {})", url),
            Error::InvalidPort(e) => write!(f, "Invalid port: {}", e),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::InvalidPort(value)
    }
}
