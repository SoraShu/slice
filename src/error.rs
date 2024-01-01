use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    ParseRangeError(ParseRangeError),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO error: {}", e),
            Error::ParseIntError(e) => write!(f, "Parse int error: {}", e),
            Error::ParseRangeError(e) => write!(f, "Parse range error: {}", e),
        }
    }
}

#[derive(Debug)]
pub struct ParseRangeError;

impl fmt::Display for ParseRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid range")
    }
}
