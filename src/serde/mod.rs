//! Serde (de)serialization for [`crate::prelude::Codec`]
mod de;
mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::Codec;
    use serde_test::{assert_tokens, Configure, Token};

    #[test]
    fn test_serde_binary() {
        let c = Codec::Ed25519Pub;
        assert_tokens(&c.compact(), &[Token::BorrowedBytes(&[0xED, 0x01])])
    }

    #[test]
    fn test_serde_readable() {
        let c = Codec::Ed25519Pub;
        assert_tokens(&c.readable(), &[Token::BorrowedStr("ed25519-pub")])
    }
}
