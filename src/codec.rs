#![allow(missing_docs)]
use crate::Error;
use multitrait::{EncodeInto, TryDecodeFrom};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

macro_rules! build_codec_enum {
    {$( $val:expr => $var:ident, )*} => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy)]

        /// Codecs from the multicodec table
        pub enum Codec {
            $( $var, )*
            Unknown(u64),
        }

        use Codec::*;

        impl Codec {
            /// Get the base code. NOTE: these are NOT varuint encoded
            pub fn code(&self) -> u64 {
                match *self {
                    $( $var => $val, )*
                    Unknown(code) => code,
                }
            }

            /// Convert a code to a base.
            pub fn from_code(code: u64) -> Result<Self, Error> {
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

impl Hash for Codec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let v: Vec<u8> = self.clone().into();
        v.hash(state);
    }
}

impl Into<Vec<u8>> for Codec {
    fn into(self) -> Vec<u8> {
        self.code().encode_into()
    }
}

impl Into<u64> for Codec {
    fn into(self) -> u64 {
        self.code()
    }
}

impl TryFrom<u8> for Codec {
    type Error = Error;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        Codec::from_code(code as u64)
    }
}

impl TryFrom<u16> for Codec {
    type Error = Error;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Codec::from_code(code as u64)
    }
}

impl TryFrom<u32> for Codec {
    type Error = Error;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        Codec::from_code(code as u64)
    }
}

impl TryFrom<u64> for Codec {
    type Error = Error;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        Codec::from_code(code)
    }
}

impl<'a> TryFrom<&'a [u8]> for Codec {
    type Error = Error;

    fn try_from(s: &'a [u8]) -> Result<Codec, Error> {
        let (code, _) = u64::try_decode_from(s)?;
        Codec::try_from(code)
    }
}

impl<'a> TryDecodeFrom<'a> for Codec {
    type Error = Error;

    fn try_decode_from(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let (code, ptr) = u64::try_decode_from(bytes)?;
        Ok((Codec::try_from(code)?, ptr))
    }
}

impl fmt::Debug for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - (0x{:x})", self.as_str(), self.code())
    }
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
        assert_eq!(0xEDu64, Ed25519Pub.into());
    }

    #[test]
    fn test_string_conversion() {
        assert_eq!("Ed25519Pub", Ed25519Pub.as_str());
    }

    #[test]
    fn test_encode_into() {
        let v: Vec<u8> = Ed25519Pub.into();
        assert_eq!(vec![0xED, 0x01], v);
    }

    #[test]
    fn test_unknown() {
        assert_eq!(Unknown(0xDEAD), Codec::from_code(0xDEAD).unwrap());
    }
}

include!("table_gen.rs");
