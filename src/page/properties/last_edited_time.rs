use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#last-edited-time>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"last_edited_time"`
/// - `$.['*'].last_edited_time`: The date and time that the page was last edited.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example last_edited_time page property value
///
/// ```json
/// {
///   "Last edited time": {
///     "id": "sv%3Fi",
///     "type": "last_edited_time",
///     "last_edited_time": "2024-04-03T10:55:00.000Z"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageLastEditedTimeProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// The date and time that the page was last edited.
    pub last_edited_time: String,
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
    fn deserialize_page_last_edited_time_property() {
        let json_data = r#"
        {
            "Last edited time": {
                "id": "sv%3Fi",
                "type": "last_edited_time",
                "last_edited_time": "2024-04-03T10:55:00.000Z"
            }
        }
        "#;

        let last_edited_time_map = serde_json::from_str::<
            std::collections::HashMap<String, PageLastEditedTimeProperty>,
        >(json_data)
        .unwrap();

        let last_edited_by = last_edited_time_map.get("Last edited time").unwrap();

        assert_eq!(last_edited_by.id, "sv%3Fi");
        assert_eq!(last_edited_by.last_edited_time, "2024-04-03T10:55:00.000Z");
    }
}
