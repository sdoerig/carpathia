use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum AdrError {
    #[error("Error: {0}")]
    GenericError(String),
    #[error("Error: {0}")]
    NoDbObjectsDiscovered(String),
}
