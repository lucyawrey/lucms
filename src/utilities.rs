use std::{fmt::{Display, Formatter, Result}, error::Error};

#[derive(Debug)]
pub struct StrError {
    msg: String,
}

impl StrError {
    pub fn new(msg: &str) -> StrError {
        StrError { msg: msg.to_string() }
    }
}

impl Display for StrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for StrError {}
