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
pub use error::Error;

/// Codec enum definition from the table
pub mod codec;

/// Serde serialization
#[cfg(feature = "serde")]
pub mod serde;

/// ...and in the darkness bind them
pub mod prelude {
    pub use super::{codec::*, error::*};

    // re-exports
    pub use multitrait::prelude::{EncodeInto, TryDecodeFrom};
}
