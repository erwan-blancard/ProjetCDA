use std::ops::Deref;
use serde::{de, Serialize};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct LimitedString<const MAX_LENGTH: usize>(String);

impl<const MAX_LENGTH: usize> Deref for LimitedString<MAX_LENGTH> {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, const MAX_LENGTH: usize> de::Deserialize<'de> for LimitedString<MAX_LENGTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        <String as de::Deserialize>::deserialize(deserializer).and_then(|inner| {
            if inner.len() > MAX_LENGTH {
                Err(de::Error::invalid_length(inner.len(), &"an integer lower than the maximum"))
            } else {
                Ok(Self(inner))
            }
        })
    }
}