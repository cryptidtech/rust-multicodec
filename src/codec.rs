#![allow(missing_docs)]
use crate::error::Error;
use multiutil::TryDecodeFrom;
use std::fmt;
use unsigned_varint::{decode, encode};

macro_rules! build_codec_enum {
    {$( $val:expr => $var:ident, )*} => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]

        /// Codecs from the multicodec table
        pub enum Codec {
            $( $var, )*
            Unknown(u128),
        }

        use Codec::*;

        impl Codec {
            /// Get the base code. NOTE: these are NOT varuint encoded
            pub fn code(&self) -> u128 {
                match *self {
                    $( $var => $val, )*
                    Unknown(code) => code,
                }
            }

            /// Convert a code to a base.
            pub fn from_code(code: u128) -> Result<Codec, Error> {
                match code {
                    $( $val => Ok($var), )*
                    _ => Ok(Unknown(code)),
                }
            }

            /// Convert a codec to &str
            pub fn as_str(&self) -> &str {
                match *self {
                    $( $var => stringify!($var), )*
                    Unknown(_) => "Unknown",
                }
            }
        }
    }
}

impl Default for Codec {
    fn default() -> Self {
        Raw
    }
}

impl Into<Vec<u8>> for Codec {
    fn into(self) -> Vec<u8> {
        let mut buf = encode::u128_buffer();
        encode::u128(self.code(), &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in buf {
            v.push(b);
            if decode::is_last(b) {
                break;
            }
        }
        v
    }
}

impl Into<u128> for Codec {
    fn into(self) -> u128 {
        self.code()
    }
}

impl TryFrom<u128> for Codec {
    type Error = Error;

    fn try_from(code: u128) -> Result<Self, Self::Error> {
        Codec::from_code(code)
    }
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (0x{:x})", self.as_str(), self.code())
    }
}

impl<'a> TryDecodeFrom<'a> for Codec {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let (code, data) = decode::u128(bytes).map_err(|e| Error::UnsignedVarintDecode(e))?;
        let codec: Codec = code.try_into()?;
        Ok((codec, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_code() {
        assert_eq!(Ed25519Pub, Codec::from_code(0xED).unwrap());
    }

    #[test]
    fn test_to_code() {
        assert_eq!(0xED, Ed25519Pub.code());
    }

    #[test]
    fn test_into_code() {
        assert_eq!(0xEDu128, Ed25519Pub.into());
    }

    #[test]
    fn test_string_conversion() {
        assert_eq!("Ed25519Pub", Ed25519Pub.as_str());
    }

    #[test]
    fn test_unknown() {
        assert_eq!(Unknown(0xDEAD), Codec::from_code(0xDEAD).unwrap());
    }
}

include!("table_gen.rs");
