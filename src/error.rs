use thiserror::Error;

/// Errors created by this library
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Multiutil error
    #[error(transparent)]
    Multiutil(#[from] multiutil::Error),
}
