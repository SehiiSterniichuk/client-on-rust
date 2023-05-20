use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct CustomError {
    pub(crate) message: String,
}

impl CustomError {
    pub(crate) fn new(message: &str) -> CustomError {
        CustomError {
            message: message.to_string(),
        }
    }
}
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {
}

