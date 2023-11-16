use crate::prelude::{Codec, TryDecodeFrom};
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
                fmt.write_str("u64 multicodec sigil")
            }

            #[inline]
            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let c = Codec::try_from(v).map_err(|e| de::Error::custom(e.to_string()))?;
                Ok(c)
            }

            #[inline]
            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let c = Codec::try_from(v).map_err(|e| de::Error::custom(e.to_string()))?;
                Ok(c)
            }

            #[inline]
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let c = Codec::try_from(v).map_err(|e| de::Error::custom(e.to_string()))?;
                Ok(c)
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let c = Codec::try_from(v).map_err(|e| de::Error::custom(e.to_string()))?;
                Ok(c)
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let (c, _) =
                    Codec::try_decode_from(v).map_err(|e| de::Error::custom(e.to_string()))?;
                Ok(c)
            }
        }

        deserializer.deserialize_any(CodecVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Codec;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_de() {
        let c = Codec::Ed25519Pub;
        assert_de_tokens(&c, &[Token::Bytes(&[0xED, 0x01])])
    }
}
