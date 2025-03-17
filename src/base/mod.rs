use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NetRouteError {
    pub message: String,
}

impl fmt::Display for NetRouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for NetRouteError {}

#[allow(dead_code)]
impl NetRouteError {
    pub fn new(message: impl Into<String>) -> Self {
        NetRouteError {
            message: message.into(),
        }
    }

    // Optional: Add conversion from other error types
    pub fn from_err<E: Error>(err: E) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

// Optional: Add From implementation for String and &str
impl From<String> for NetRouteError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for NetRouteError {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
