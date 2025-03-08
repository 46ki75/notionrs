use chrono::TimeZone;
use serde::{Deserialize, Deserializer, Serialize};

/// <https://developers.notion.com/reference/page-property-values#verification>
///
/// The verification status of a page in a wiki database. Pages can be verified or unverified,
/// and verifications can have an optional expiration date set.
///
/// The verification status cannot currently be set or updated via the public API.
///
/// - `$.['*'].state`: The verification state of the page. `"verified"` or `"unverified"`.
/// - `$.['*'].verified_by`: Always `"url"`
/// - `$.['*'].date?`: If the page if verified, a User object will be included to indicate the user who verified the page.
///                   If an expiration date is set for the verification, an end date (end) will be included.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PageVerificationProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    pub verification: PageVerificationPropertyParameter,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PageVerificationPropertyParameter {
    /// The verification state of the page. `"verified"` or `"unverified"`.
    pub state: PageVerificationState,

    pub verified_by: Option<crate::user::User>,

    pub date: Option<PageVerificationDate>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PageVerificationState {
    Verified,
    #[default]
    Unverified,
    Expired,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct PageVerificationDate {
    /// A date, with an optional time.
    #[serde(deserialize_with = "deserialize_date_or_datetime")]
    pub start: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// A string representing the end of a date range.
    /// If the value is null, then the date value is not a range.
    #[serde(deserialize_with = "deserialize_date_or_datetime")]
    pub end: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Always `null`. The time zone is already included in the formats of start and end times.
    #[serde(skip_deserializing)]
    pub time_zone: Option<String>,
}

fn deserialize_date_or_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = Option::<String>::deserialize(deserializer)?;

    if let Some(date_str) = date_str {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            return Ok(Some(
                chrono::FixedOffset::east_opt(0).unwrap().from_utc_datetime(
                    &date
                        .and_hms_opt(0, 0, 0)
                        .ok_or_else(|| serde::de::Error::custom("Invalid time"))?,
                ),
            ));
        }
        if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&date_str) {
            return Ok(Some(datetime));
        }
        return Err(serde::de::Error::custom("Invalid date or datetime format"));
    }

    Ok(None)
}

impl PageVerificationProperty {
    pub fn state(&mut self, state: PageVerificationState) -> &mut Self {
        self.verification.state = state;
        self
    }

    pub fn verified_by(&mut self, verified_by: crate::user::User) -> &mut Self {
        self.verification.verified_by = Some(verified_by);
        self
    }

    pub fn date(&mut self, date: PageVerificationDate) -> &mut Self {
        self.verification.date = Some(date);
        self
    }
}

impl std::fmt::Display for PageVerificationProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.verification.state {
                PageVerificationState::Verified => "verified",
                PageVerificationState::Unverified => "unverified",
                PageVerificationState::Expired => "expired",
            }
        )
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_page_verification_property() {
        let json_data = r#"
        {
            "id": "%3DP%7CC",
            "type": "verification",
            "verification": {
                "state": "verified",
                "verified_by": {
                    "object": "user",
                    "id": "174984bc-2b3e-408f-97fd-fa5ff989e907",
                    "name": "<masked>",
                    "avatar_url": "https://example.com/",
                    "type": "person",
                    "person": {
                        "email": "<masked>@example.com"
                    }
                },
                "date": {
                    "start": "2024-12-11T15:00:00.000Z",
                    "end": "2024-12-18T15:00:00.000Z",
                    "time_zone": null
                }
            }
        }
        "#;

        let page_verification: PageVerificationProperty = serde_json::from_str(json_data).unwrap();

        assert_eq!(
            page_verification.verification.state,
            PageVerificationState::Verified
        );
    }
}
