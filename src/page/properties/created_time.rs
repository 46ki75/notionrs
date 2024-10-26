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
#[derive(Debug, Deserialize, Serialize)]
pub struct PageCreatedTimeProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: Option<String>,

    /// The date and time that the page was created.
    ///The created_time value can’t be updated.
    pub created_time: chrono::DateTime<chrono::Utc>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use chrono::TimeZone;

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

        let expected_created_time = chrono::Utc.with_ymd_and_hms(2024, 4, 3, 10, 55, 0).unwrap();
        assert_eq!(created_time.created_time, expected_created_time);
    }
}
