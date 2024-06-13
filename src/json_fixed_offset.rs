use chrono::{FixedOffset, offset::Offset};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(
    offset: &Option<FixedOffset>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match offset {
        Some(o) => o.fix().local_minus_utc().serialize(serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<FixedOffset>, D::Error> {
    let offset_seconds: Option<i32> = Option::deserialize(deserializer)?;

    match offset_seconds {
        Some(seconds) => Ok(Some(
            FixedOffset::east_opt(seconds).ok_or_else(|| D::Error::custom("Invalid offset"))?,
        )),
        None => Ok(None),
    }
}
