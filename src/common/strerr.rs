use std::error;
use std::fmt;
use std::fmt::{Debug, Display};

// 创造自己的string error 和上面配合使用
#[derive(Debug)]
pub struct StrError {
    details: String,
}

impl StrError {
    pub fn new(msg: &str) -> StrError {
        StrError {
            details: msg.to_string(),
        }
    }
}

impl Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for StrError {
    fn description(&self) -> &str {
        &self.details
    }
}