use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

/// Id type for bypass Derpibooru API bug.
///
/// Derpibooru API has bug, different API methods returns `uploader_id` and `tag_ids` either as a string or as a number.
/// This type has custom `serde::Deserialize` implementation for cast it from string and number.
#[derive(Debug)]
pub struct Id(pub u64);

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = Id;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("ID as a number or string")
            }
            fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Id(id))
            }
            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                id.parse().map(Id).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}
