use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(#[serde(with = "super::details::serde_timestamp")] pub DateTime<Utc>);

///////////////////////////////////////////////////////////////////////////////////////////////////
/// This uses the chrono crate to serialize and deserialize JSON data
/// containing a Gerrit's custom timestamp format.
/// The with attribute (as in #[serde(with="serde_timestamp")]) is used
/// to provide the logic for handling the custom representation for DateTime<Utc>.
pub mod serde_timestamp {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Gerrit timestamps are given in UTC and have the format "'yyyy-mm-dd hh:mm:ss.fffffffff'"
    /// where "'ffffffffff'" represents nanoseconds.
    const GERRIT_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f";

    /// Serialize a DateTime<Utc> using the GERRIT_FORMAT specified above.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(GERRIT_FORMAT));
        serializer.serialize_str(&s)
    }

    /// Deserialize a DateTime<Utc> using the GERRIT_FORMAT specified above.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, GERRIT_FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
