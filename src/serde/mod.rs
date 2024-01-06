//! Serde (de)serialization for [`crate::prelude::Codec`]
mod de;
mod ser;

#[cfg(test)]
mod tests {
    use crate::prelude::Codec;
    use serde::{Deserialize, Serialize};
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper {
        v: Vec<Codec>,
    }

    #[test]
    fn test_serde_macros() {
        let w1 = Wrapper {
            v: vec![Codec::Ed25519Pub, Codec::Secp256K1Pub, Codec::Bls12381G1Pub],
        };
        let b = serde_cbor::to_vec(&w1).unwrap();
        let w2 = serde_cbor::from_slice(b.as_slice()).unwrap();
        assert_eq!(w1, w2);
        let s = serde_json::to_string_pretty(&w1).unwrap();
        println!("{}", s);
        let w3 = serde_json::from_str(&s).unwrap();
        assert_eq!(w1, w3);
    }
}
