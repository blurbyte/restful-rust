use serde::de::{Deserializer, Error as DeserializerError, Unexpected};
use serde::ser::{Error as SerializerError, Serializer};
use serde::Deserialize;

pub fn deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u8::deserialize(deserializer)?;

    if value > 100 {
        return Err(DeserializerError::invalid_value(
            Unexpected::Unsigned(u64::from(value)),
            &"rating must be a number between 0 and 100",
        ));
    }

    Ok(value)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize<S>(value: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if *value > 100 {
        return Err(SerializerError::custom("rating must be a number between 0 and 100"));
    }

    serializer.serialize_u8(*value)
}
