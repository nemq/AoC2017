use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SimpleError {
   msg: String,
}

impl SimpleError {
    pub fn new(msg: &str) -> SimpleError {
        SimpleError{msg: String::from(msg)}
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleError: {}", self.msg)
    }
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        &self.msg
    }
}

