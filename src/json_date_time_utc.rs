use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(time: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error> {
    match time {
        Some(t) => t.to_rfc3339().serialize(serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error> {
    let time: Option<&str> = Option::deserialize(deserializer)?;
    
    match time {
        Some(t) => {
            let dt = DateTime::parse_from_rfc3339(&t)
                .map_err(D::Error::custom)?;
            Ok(Some(dt.naive_utc()))
        },
        None => Ok(None),
    }
}
