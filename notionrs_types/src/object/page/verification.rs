use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#verification>
///
/// The verification status of a page in a wiki database. Pages can be verified or unverified,
/// and verifications can have an optional expiration date set.
///
/// - `$.['*'].state`: The verification state of the page. `"verified"` or `"unverified"`.
/// - `$.['*'].verified_by`: A User object indicating the user who verified the page.
///   If the page is unverified, this will be `null`.
/// - `$.['*'].date?`: If the page is verified and an expiration date is set, an end date (`end`) will be included.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PageVerificationProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    pub verification: PageVerificationPropertyParameter,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PageVerificationPropertyParameter {
    /// The verification state of the page. `"verified"` or `"unverified"`.
    pub state: PageVerificationState,

    pub verified_by: Option<crate::object::user::User>,

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PageVerificationDate {
    /// A date, with an optional time.
    pub start: Option<crate::object::date::DateOrDateTime>,

    /// A string representing the end of a date range.
    /// If the value is null, then the date value is not a range.
    pub end: Option<crate::object::date::DateOrDateTime>,

    /// Always `null`. The time zone is already included in the formats of start and end times.
    #[serde(skip_deserializing)]
    pub time_zone: Option<String>,
}

impl PageVerificationProperty {
    pub fn state(&mut self, state: PageVerificationState) -> &mut Self {
        self.verification.state = state;
        self
    }

    pub fn verified_by(&mut self, verified_by: crate::object::user::User) -> &mut Self {
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

    #[test]
    fn page_verification_setters_and_display() {
        use crate::object::date::DateOrDateTime;
        let mut p = PageVerificationProperty::default()
            .id("id")
            .verification(PageVerificationPropertyParameter::default());
        p.state(PageVerificationState::Verified);
        p.verified_by(crate::object::user::User::default());
        p.date(PageVerificationDate::default());
        assert_eq!(p.to_string(), "verified");

        let mut p2 = PageVerificationProperty::default();
        p2.state(PageVerificationState::Unverified);
        assert_eq!(p2.to_string(), "unverified");

        let mut p3 = PageVerificationProperty::default();
        p3.state(PageVerificationState::Expired);
        assert_eq!(p3.to_string(), "expired");

        let _date = PageVerificationDate::default()
            .start(DateOrDateTime::Date(
                time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
            ))
            .end(DateOrDateTime::Date(
                time::Date::from_calendar_date(2024, time::Month::January, 2).unwrap(),
            ))
            .time_zone("UTC");

        let _param = PageVerificationPropertyParameter::default()
            .state(PageVerificationState::Verified)
            .verified_by(crate::object::user::User::default())
            .date(PageVerificationDate::default());
    }
}
