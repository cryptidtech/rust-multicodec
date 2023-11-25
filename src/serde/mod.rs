//! Serde (de)serialization for [`crate::prelude::Codec`]
mod de;
mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::Codec;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_serde() {
        let c = Codec::Ed25519Pub;
        assert_tokens(&c, &[Token::Bytes(&[0xED, 0x01])])
    }
}
