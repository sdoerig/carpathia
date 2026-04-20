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

impl From<ErrorNumber> for i32 {
    fn from(error_type: ErrorNumber) -> i32 {
        match error_type {
            ErrorNumber::CacheFileError(code) => code,
            ErrorNumber::ConfigFileError(code) => code,
            ErrorNumber::GenerationError(code) => code,
            ErrorNumber::Other(code) => code,
            ErrorNumber::Success(code) => code,
        }
    }
}

impl Error for CarpathiaError {}
