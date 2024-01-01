use std::{error, fmt};

#[derive(Debug)]
pub struct ParseRangeError;

impl fmt::Display for ParseRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid range")
    }
}

impl error::Error for ParseRangeError {}
