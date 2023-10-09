#![allow(missing_docs)]

use crate::error::Error;
use std::fmt;
use unsigned_varint::{decode, encode};

macro_rules! build_codec_enum {
    {$( $val:expr => $var:ident, )*} => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]

        /// Codecs from the multicodec table
        pub enum Codec {
            $( $var, )*
            Unknown(u64),
        }

        use Codec::*;

        impl Codec {
            /// Get the base code.
            pub fn code(&self) -> u64 {
                match *self {
                    $( $var => $val, )*
                    Unknown(code) => code,
                }
            }

            /// Convert a code to a base.
            pub fn from_code(code: u64) -> Result<Codec, Error> {
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
        let mut buf = [0u8; 10];
        encode::u64(self.code(), &mut buf);
        let mut v: Vec<u8> = Vec::new();
        for b in &buf {
            v.push(*b);
            if decode::is_last(*b) {
                break;
            }
        }
        v
    }
}

impl TryFrom<u64> for Codec {
    type Error = Error;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        Codec::from_code(code)
    }
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (0x{:x})", self.as_str(), self.code())
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
    fn test_string_conversion() {
        assert_eq!("Ed25519Pub", Ed25519Pub.as_str());
    }

    #[test]
    fn test_unknown() {
        assert_eq!(Unknown(0xDEAC), Codec::from_code(0xDEAD).unwrap());
    }
}

include!("table_gen.rs");
