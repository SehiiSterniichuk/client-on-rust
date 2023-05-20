use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ResponseType {
    OK,
    BadRequest,
}

impl fmt::Display for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseType::OK => write!(f, "OK"),
            ResponseType::BadRequest => write!(f, "BAD_REQUEST"),
        }
    }
}
