use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#last-edited-by>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"last_edited_by"`
/// - `$.['*'].last_edited_by`: A [user object](https://developers.notion.com/reference/user)
///   containing information about the user who last updated the page.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example last_edited_by page property value
///
/// ```json
/// {
///   "Last edited by": {
///     "id": "fR4s",
///     "type": "last_edited_by",
///     "last_edited_by": {
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct PageLastEditedByProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A [user object](https://developers.notion.com/reference/user)
    /// containing information about the user who last updated the page.
    pub last_edited_by: crate::object::user::User,
}

impl std::fmt::Display for PageLastEditedByProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.last_edited_by)
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
    fn deserialize_page_last_edited_by_property() {
        let json_data = r#"
        {
            "Last edited by": {
                "id": "fR4s",
                "type": "last_edited_by",
                "last_edited_by": {
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

        let last_edited_by_map = serde_json::from_str::<
            std::collections::HashMap<String, PageLastEditedByProperty>,
        >(json_data)
        .unwrap();

        let last_edited_by = last_edited_by_map.get("Last edited by").unwrap();

        assert_eq!(last_edited_by.id, Some("fR4s".to_string()));

        let person = &last_edited_by.last_edited_by;

        assert_eq!(person.object, "user");
        assert_eq!(person.id, "cb497a8c-1c30-4c22-87af-f8b0c1ee7389");
        assert_eq!(person.name, Some("Sam".to_string()));
        assert_eq!(person.avatar_url, None);
        assert_eq!(person.r#type, Some("person".to_string()));
        assert_eq!(
            person.person.as_ref().unwrap().email,
            Some("info@example.com".to_string())
        );
    }
}
