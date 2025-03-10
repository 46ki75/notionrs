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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PageStatusProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Select object
    pub status: crate::object::select::Select,
}

impl std::fmt::Display for PageStatusProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.status)
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

        assert_eq!(status.id, Some("xx%7Cd".to_string()));
        assert_eq!(
            status.status.id,
            Some("4a1accbf-6716-4cf2-9034-5877581fc5f6".to_string())
        );
        assert_eq!(status.status.name, "Not started");
        assert_eq!(
            status.status.color,
            Some(crate::object::select::SelectColor::Default)
        );
    }
}
