use std::error::Error;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorNumber {
    CacheFileError,
    CacheFileReadError,
    ConfigFileError,
    DatabaseConnectionError,
    GenerationError,
    Other,
    InvalidConstraintType,
    InvalidObjectType,
    InvalidPoolType,
    InvalidConfiguration,
    NoDbObjectsDiscovered,
    NoTemplatesFound,
    FileWriteError,
    PathCanonicalizationError,
    PathEscapesOutputDir,
    TemplateWriteError,
    ErrorWritingInitTemplate,
    Success,
}

#[derive(Debug, Clone)]
pub struct CarpathiaError {
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
            ErrorNumber::CacheFileReadError => 4,
            ErrorNumber::ConfigFileError => 2,
            ErrorNumber::DatabaseConnectionError => 5,
            ErrorNumber::GenerationError => 1,
            ErrorNumber::Other => 20,
            ErrorNumber::InvalidConstraintType => 6,
            ErrorNumber::InvalidObjectType => 7,
            ErrorNumber::InvalidPoolType => 8,
            ErrorNumber::InvalidConfiguration => 9,
            ErrorNumber::NoDbObjectsDiscovered => 10,
            ErrorNumber::NoTemplatesFound => 11,
            ErrorNumber::FileWriteError => 12,
            ErrorNumber::PathCanonicalizationError => 13,
            ErrorNumber::PathEscapesOutputDir => 14,
            ErrorNumber::TemplateWriteError => 15,
            ErrorNumber::ErrorWritingInitTemplate => 16,
            ErrorNumber::Success => 0,
        }
    }
}

impl Error for CarpathiaError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_number_to_i32() {
        assert_eq!(i32::from(ErrorNumber::CacheFileError), 3);
        assert_eq!(i32::from(ErrorNumber::DatabaseConnectionError), 5);
        assert_eq!(i32::from(ErrorNumber::Success), 0);
        assert_eq!(i32::from(ErrorNumber::Other), 20);
        assert_eq!(i32::from(ErrorNumber::InvalidConfiguration), 9);
        assert_eq!(i32::from(ErrorNumber::NoTemplatesFound), 11);
        assert_eq!(i32::from(ErrorNumber::FileWriteError), 12);
        assert_eq!(i32::from(ErrorNumber::PathCanonicalizationError), 13);
        assert_eq!(i32::from(ErrorNumber::PathEscapesOutputDir), 14);
        assert_eq!(i32::from(ErrorNumber::TemplateWriteError), 15);
        assert_eq!(i32::from(ErrorNumber::ErrorWritingInitTemplate), 16);
        assert_eq!(i32::from(ErrorNumber::InvalidConstraintType), 6);
        assert_eq!(i32::from(ErrorNumber::InvalidObjectType), 7);
        assert_eq!(i32::from(ErrorNumber::InvalidPoolType), 8);
        assert_eq!(i32::from(ErrorNumber::NoDbObjectsDiscovered), 10);
        assert_eq!(i32::from(ErrorNumber::ConfigFileError), 2);
        assert_eq!(i32::from(ErrorNumber::GenerationError), 1);
        assert_eq!(i32::from(ErrorNumber::CacheFileReadError), 4);
    }

    #[test]
    fn test_carpathia_error_display() {
        let error = CarpathiaError {
            message: "Test error".into(),
            error_type: ErrorNumber::Other,
        };
        assert_eq!(format!("{}", error), "CarpathiaError: Test error");
    }
}
