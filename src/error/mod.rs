use std::error::Error;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ByteBufError {
    pub message: String,
}

impl ByteBufError {
    pub fn new(message: String) -> Self {
        ByteBufError {
            message
        }
    }

    pub fn fromIoError(err: std::io::Error) -> Self {
        ByteBufError {
            message: err.to_string()
        }
    }
}


impl fmt::Display for ByteBufError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ByteBufError {
    fn description(&self) -> &str {
        &self.message
    }
}


impl From<std::io::Error> for ByteBufError {
    fn from(error: std::io::Error) -> Self {
        ByteBufError{
            message: error.to_string(),
        }
    }
}

