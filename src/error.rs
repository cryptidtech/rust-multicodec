use thiserror::Error;

/// Errors created by this library
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Formatting error
    #[error("fmt error {0}")]
    Fmt(#[from] std::fmt::Error),

    /// Multiutil error
    #[error(transparent)]
    MultiUtilError(#[from] multiutil::Error),
}
