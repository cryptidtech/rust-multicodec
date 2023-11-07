//!
#![warn(missing_docs)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

/// Errors produced by this library
pub mod error;

/// Codec enum definition from the table
pub mod codec;

/// ...and in the darkness bind them
pub mod prelude {
    use super::*;

    pub use codec::*;
    pub use error::*;
}
