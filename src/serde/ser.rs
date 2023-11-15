use crate::prelude::Codec;
use multiutil::EncodeInto;
use serde::ser;

/// Serialize instances of [`crate::prelude::Codec`] into varuint encoded bytes
impl ser::Serialize for Codec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_bytes(self.encode_into().as_slice())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Codec;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_ser() {
        let c = Codec::Ed25519Pub;
        assert_ser_tokens(&c, &[Token::Bytes(&[0xED, 0x01])]);
    }
}
