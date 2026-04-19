use std::error::Error;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ErrorNumber {
    CacheFileError(i32),
    ConfigFileError(i32),
    GenerationError(i32),
    Other(i32),
    Success(i32),
}

#[derive(Debug, Clone)]
pub(crate) struct CarpathiaError {
    pub message: String,
    pub error_type: ErrorNumber,
}

impl fmt::Display for CarpathiaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CarpathiaError: {}", self.message)
    }
}

impl Error for CarpathiaError {}
