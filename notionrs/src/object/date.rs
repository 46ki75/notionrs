use serde::{Deserialize, Serialize};

/// Represents either a `Date` (`YYYY-MM-DD`) or a full `OffsetDateTime` (`RFC 3339`) value.
///
/// This enum is useful when the input format can vary between:
/// - A date-only string, like `"2024-05-01"`
/// - A full datetime string with offset, like `"2024-05-01T00:00:00Z"`
///
/// When deserializing, the format determines which variant is used:
/// - `"YYYY-MM-DD"` → [`DateOrDateTime::Date`]
/// - RFC 3339 datetime → [`DateOrDateTime::DateTime`]
///
/// ### Examples
/// ```json
/// "2024-05-01"                  // => Date(...)
/// "2024-05-01T00:00:00+00:00"   // => DateTime(...)
/// ```
///
/// This enum uses custom (de)serializers to support both formats.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(untagged)]
pub enum DateOrDateTime {
    /// A date-only value (e.g. `"2024-05-01"`).
    #[serde(with = "crate::serde::date")]
    Date(time::Date),

    /// A full date-time with offset (RFC 3339 format).
    #[serde(with = "time::serde::rfc3339")]
    DateTime(time::OffsetDateTime),
}

impl std::fmt::Display for DateOrDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = match self {
            DateOrDateTime::Date(date) => date.to_string(),
            DateOrDateTime::DateTime(offset_date_time) => offset_date_time.to_string(),
        };

        write!(f, "{}", date)
    }
}

impl Default for DateOrDateTime {
    fn default() -> Self {
        Self::DateTime(time::OffsetDateTime::now_utc())
    }
}
