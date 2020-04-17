use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AppError {
    description: String
}

impl AppError {
    pub fn new(err: &Error) -> AppError {
        AppError {
            description: format!("{:?}", err)
        }
    }

    pub fn new_from_string(err: &str) -> AppError {
        AppError {
            description: format!("{:?}", err)
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, ": {}", self.description)?;
        Ok(())
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        "timed out waiting for connection"
    }
}
