#![allow(missing_docs)]

use crate::error::Error;

macro_rules! build_codec_enum {
    {$( $val:expr => $var:ident, )*} => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]

        /// Codecs from the multicodec table
        pub enum Codec {
            $( $var, )*
        }

        use Codec::*;

        impl Codec {
            /// Get the base code.
            pub fn code(&self) -> u64 {
                match *self {
                    $( $var => $val, )*
                }
            }

            /// Convert a code to a base.
            pub fn from_code(code: u64) -> Result<Codec, Error> {
                match code {
                    $( $val => Ok($var), )*
                    _ => Err(Error::InvalidCodec(code).into()),
                }
            }
        }
    }
}

impl TryFrom<u64> for Codec {
    type Error = Error;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        Codec::from_code(code)
    }
}

include!("table_gen.rs");
