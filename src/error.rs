use thiserror::Error;

/// Errors created by this library
#[derive(Clone, Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Multitrait error
    #[error(transparent)]
    Multitrait(#[from] multitrait::Error),
    /// Invalid codec name
    #[error("Invalid multicodec name {0}")]
    InvalidName(String),
    /// Invalid codec value
    #[error("Invalid multicodec value {0}")]
    InvalidValue(u64),
}
