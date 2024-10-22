use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#status>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"status"`
/// - `$.['*'].status`: Select object
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example status page property value
///
/// ```json
/// {
///   "Status": {
///     "type": "status",
///     "id": "xx%7Cd",
///     "status": {
///       "id": "4a1accbf-6716-4cf2-9034-5877581fc5f6",
///       "name": "Not started",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageStatusProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// Select object
    pub status: crate::others::select::Select,
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
    fn deserialize_status_property() {
        let json_data = r#"
        {
            "Status": {
                "type": "status",
                "id": "xx%7Cd",
                "status": {
                    "id": "4a1accbf-6716-4cf2-9034-5877581fc5f6",
                    "name": "Not started",
                    "color": "default"
                }
            }
        }
        "#;

        let status_map = serde_json::from_str::<
            std::collections::HashMap<String, PageStatusProperty>,
        >(json_data)
        .unwrap();

        let status = status_map.get("Status").unwrap();

        assert_eq!(status.id, "xx%7Cd");
        assert_eq!(status.status.id, "4a1accbf-6716-4cf2-9034-5877581fc5f6");
        assert_eq!(status.status.name, "Not started");
        assert_eq!(
            status.status.color,
            crate::others::select::SelectColor::Default
        );
    }
}
