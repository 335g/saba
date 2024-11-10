use alloc::string::String;
use core::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidUrl(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl(url) => write!(f, "Invalid Url: {}", url),
        }
    }
}
