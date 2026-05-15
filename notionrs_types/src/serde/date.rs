//! Defines custom serializers and deserializers for `time::Date`
//! using `YYYY-MM-DD` format.

use serde::{Deserialize, Deserializer, Serializer, de::Error};

/// Serializes a `Date` into a `YYYY-MM-DD` string.
pub fn serialize<S>(value: &time::Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = time::macros::format_description!("[year]-[month]-[day]");
    let s = value.format(&format).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

/// Deserializes a `YYYY-MM-DD` string into a `Date`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<time::Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format = time::macros::format_description!("[year]-[month]-[day]");
    time::Date::parse(&s, &format).map_err(D::Error::custom)
}

#[cfg(test)]
mod unit_tests {
    #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
    struct Wrapper {
        #[serde(with = "crate::serde::date")]
        d: time::Date,
    }

    #[test]
    fn serde_date_module_roundtrip() {
        let json = r#"{"d":"2024-04-04"}"#;
        let w: Wrapper = serde_json::from_str(json).unwrap();
        assert_eq!(
            w.d,
            time::Date::from_calendar_date(2024, time::Month::April, 4).unwrap()
        );
        let back = serde_json::to_string(&w).unwrap();
        assert_eq!(back, json);

        let bad = serde_json::from_str::<Wrapper>(r#"{"d":"not-a-date"}"#);
        assert!(bad.is_err());
    }
}
