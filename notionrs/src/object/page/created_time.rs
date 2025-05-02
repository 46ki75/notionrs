use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#created-time>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"created_time"`
/// - `$.['*'].created_time`: The date and time that the page was created.
///                           The created_time value can’t be updated.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example created_time page property value
///
/// ```json
/// {
///   "Created time": {
///     "id": "sv%3Fi",
///     "type": "created_time",
///     "created_time": "2024-04-03T10:55:00.000Z"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct PageCreatedTimeProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The date and time that the page was created.
    /// The created_time value can’t be updated.
    #[serde(with = "time::serde::rfc3339")]
    pub created_time: time::OffsetDateTime,
}

impl Default for PageCreatedTimeProperty {
    fn default() -> Self {
        self::PageCreatedTimeProperty {
            id: None,
            created_time: time::OffsetDateTime::now_utc(),
        }
    }
}

impl std::fmt::Display for PageCreatedTimeProperty {
    /// display the created time in RFC3339 format
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.created_time
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap()
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
    fn deserialize_page_created_time_property() {
        let json_data = r#"
        {
            "Created time": {
                "id": "sv%3Fi",
                "type": "created_time",
                "created_time": "2024-04-03T10:55:00.000Z"
            }
        }
        "#;

        let created_time_map = serde_json::from_str::<
            std::collections::HashMap<String, PageCreatedTimeProperty>,
        >(json_data)
        .unwrap();

        let created_time = created_time_map.get("Created time").unwrap();

        assert_eq!(created_time.id, Some("sv%3Fi".to_string()));

        let expected_created_time = time::OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2024, time::Month::April, 3).unwrap(),
            time::Time::from_hms(10, 55, 0).unwrap(),
        );
        assert_eq!(created_time.created_time, expected_created_time);
    }
}
