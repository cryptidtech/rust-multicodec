use thiserror::Error;

/// Errors created by this library
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Multitrait error
    #[error(transparent)]
    Multitrait(#[from] multitrait::prelude::Error),
}
