use crate::{codec::Codec, error::Error};
use unsigned_varint::decode;

/// Multicodec wrapper for pulling codec values from slices
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct MultiCodec<'a> {
    codec: Codec,
    data: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for MultiCodec<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let (code, data) = decode::u64(bytes).map_err(|e| Error::UnsignedVarintDecode(e))?;
        Ok(MultiCodec {
            codec: code.try_into()?,
            data,
        })
    }
}

impl Into<Vec<u8>> for MultiCodec<'_> {
    fn into(self) -> Vec<u8> {
        let mut v: Vec<u8> = self.codec.into();
        v.extend(self.data);
        v
    }
}

impl<'a> MultiCodec<'a> {
    /// create a new MultiCodec
    pub fn new(codec: Codec, data: &'a [u8]) -> MultiCodec {
        MultiCodec { codec, data }
    }

    /// get the codec
    pub fn codec(&self) -> Codec {
        self.codec
    }

    /// get the remaining slice reference
    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let codec = MultiCodec::new(Codec::Sha2256, b"EiC5TSe5k00");
        let packed: Vec<u8> = codec.into();
        let redo = MultiCodec::try_from(packed.as_slice()).unwrap();
        assert_eq!(packed, "\x12EiC5TSe5k00".as_bytes());
        assert_eq!(redo, codec);
    }

    #[test]
    fn test_to_vec() {
        let bytes = Codec::Ed25519Priv.to_vec();
        assert_eq!([0x80, 0x26], bytes.as_slice());
        let mc = MultiCodec::try_from(bytes.as_slice()).unwrap();
        assert_eq!(mc.codec(), Codec::Ed25519Priv);
    }
}
