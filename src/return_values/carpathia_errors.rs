use std::error::Error;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ErrorNumber {
    CacheFileError,
    ConfigFileError,
    GenerationError,
    Other,
    Success,
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

impl From<ErrorNumber> for i32 {
    fn from(error_type: ErrorNumber) -> i32 {
        match error_type {
            ErrorNumber::CacheFileError => 3,
            ErrorNumber::ConfigFileError => 2,
            ErrorNumber::GenerationError => 1,
            ErrorNumber::Other => 20,
            ErrorNumber::Success => 0,
        }
    }
}

impl Error for CarpathiaError {}
