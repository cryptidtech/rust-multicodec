use crate::prelude::Codec;

/// This trait exposes the codec information for multicoded types
pub trait CodecInfo {
    /// return the codec associated with this object
    fn codec(&self) -> Codec;
}
