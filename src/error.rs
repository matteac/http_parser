use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    err: String,
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.err
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        Self {
            err: msg.to_string(),
        }
    }
}
