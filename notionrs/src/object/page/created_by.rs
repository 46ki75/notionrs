use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#created-by>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"created_by"`
/// - `$.['*'].created_by`: A [user object](https://developers.notion.com/reference/user)
///   containing information about the user who created the page.
///   `created_by` can’t be updated.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example created_by page property value
///
/// ```json
/// {
///   "Created by": {
///     "id": "fR4s",
///     "type": "created_by",
///     "created_by": {
///       "object": "user",
///       "id": "cb497a8c-1c30-4c22-87af-f8b0c1ee7389",
///       "name": "Sam",
///       "avatar_url": null,
///       "type": "person",
///       "person": {
///         "email": "info@example.com"
///       }
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct PageCreatedByProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A [user object](https://developers.notion.com/reference/user)
    /// containing information about the user who created the page.
    /// `created_by` can’t be updated.
    pub created_by: crate::object::user::User,
}

impl std::fmt::Display for PageCreatedByProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.created_by)
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
    fn deserialize_page_created_by_property() {
        let json_data = r#"
        {
            "Created by": {
                "id": "fR4s",
                "type": "created_by",
                "created_by": {
                    "object": "user",
                    "id": "cb497a8c-1c30-4c22-87af-f8b0c1ee7389",
                    "name": "Sam",
                    "avatar_url": null,
                    "type": "person",
                    "person": {
                        "email": "info@example.com"
                    }
                }
            }
        }
        "#;

        let created_by_map = serde_json::from_str::<
            std::collections::HashMap<String, PageCreatedByProperty>,
        >(json_data)
        .unwrap();

        let created_by = created_by_map.get("Created by").unwrap();

        assert_eq!(created_by.id, Some("fR4s".to_string()));
    }
}
