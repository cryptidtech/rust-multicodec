use crate::prelude::Codec;
use serde::ser;

/// Serialize instances of [`crate::prelude::Codec`] into varuint encoded bytes
impl ser::Serialize for Codec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v: Vec<u8> = self.clone().into();
        serializer.serialize_bytes(v.as_slice())
    }
}
