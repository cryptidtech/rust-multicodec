use crate::prelude::Codec;
use core::fmt;
use serde::de;

/// Deserialize instances of [`crate::prelude::Codec`] from a varuint encoded
/// byte slice or a u8, u16, u32, or u64 value.
impl<'de> de::Deserialize<'de> for Codec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CodecVisitor;

        impl<'de> de::Visitor<'de> for CodecVisitor {
            type Value = Codec;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("varuint encoded multicodec sigil")
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Codec::try_from(v).map_err(|e| de::Error::custom(e.to_string()))?)
            }
        }

        deserializer.deserialize_bytes(CodecVisitor)
    }
}
