use thiserror::Error;

/// Errors created by this library
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Formatting error
    #[error("fmt error {0}")]
    Fmt(#[from] std::fmt::Error),

    /// A generic error message
    #[error("General varsig error: {0}")]
    General(&'static str),

    /// An invalid codec error
    #[error("Invalid codec value: 0x{0:x}")]
    InvalidCodec(u64),

    /// A unsigned-varint error
    #[error("Unsigned varint decode error: {0}")]
    UnsignedVarintDecode(#[from] unsigned_varint::decode::Error),
}
