use crate::Codec;
//use core::fmt;
use serde::{de, Deserialize, Deserializer};

/// Deserialize instances of [`crate::prelude::Codec`] from a varuint encoded
/// byte slice or a u8, u16, u32, or u64 value.
impl<'de> Deserialize<'de> for Codec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s: &str = Deserialize::deserialize(deserializer)?;
            Ok(Codec::try_from(s).map_err(|e| de::Error::custom(e.to_string()))?)
        } else {
            let b: &[u8] = Deserialize::deserialize(deserializer)?;
            Ok(Codec::try_from(b).map_err(|e| de::Error::custom(e.to_string()))?)
        }
    }
}
