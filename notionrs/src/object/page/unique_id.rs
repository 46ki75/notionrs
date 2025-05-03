use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#unique-id>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"unique_id"`
/// - `$.['*'].unique_id`: A unique ID assigned through auto increment
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example unique_id page property value
///
/// ```json
/// {
///   "ID": {
///     "id": "mBKy",
///     "type": "unique_id",
///     "unique_id": {
///       "prefix": "TES",
///       "number": 434
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PageUniqueIdProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing)]
    pub id: Option<String>,

    /// A unique ID assigned through auto increment
    pub unique_id: PageUniqueIdPropertyParameter,
}

/// Unique IDs can be read using the API with a GET page request,
/// but they cannot be updated with the API, since they are auto-incrementing.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PageUniqueIdPropertyParameter {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub prefix: Option<String>,

    /// The ID count (auto-incrementing).
    pub number: u64,
}

impl std::fmt::Display for PageUniqueIdProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.unique_id)
    }
}

impl std::fmt::Display for PageUniqueIdPropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.prefix {
            Some(prefix) => write!(f, "{}-{}", prefix, self.number),
            None => write!(f, "{}", self.number),
        }
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
    fn deserialize_unique_id_property() {
        let json_data = r#"
        {
            "ID": {
                "id": "mBKy",
                "type": "unique_id",
                "unique_id": {
                    "prefix": "TES",
                    "number": 434
                }
            }
        }
        "#;

        let unique_id_map = serde_json::from_str::<
            std::collections::HashMap<String, PageUniqueIdProperty>,
        >(json_data)
        .unwrap();

        let unique_id = unique_id_map.get("ID").unwrap();

        assert_eq!(unique_id.id, Some("mBKy".to_string()));
        assert_eq!(unique_id.unique_id.prefix, Some("TES".to_string()));
        assert_eq!(unique_id.unique_id.number, 434);
    }
}
