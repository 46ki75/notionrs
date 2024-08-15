use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#last-edited-by>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"last_edited_by"`
/// - `$.['*'].last_edited_by`: A [user object](https://developers.notion.com/reference/user)
///                             containing information about the user who last updated the page.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example last_edited_by page property value // TODO: documentation replace placeholder
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
#[derive(Debug, Deserialize, Serialize)]
pub struct PageLastEditedByProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// A [user object](https://developers.notion.com/reference/user)
    /// containing information about the user who last updated the page.
    pub last_edited_by: crate::user::User,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_deserialize_page_last_edited_by_property() {
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

        assert_eq!(last_edited_by.id, "fR4s");

        match &last_edited_by.last_edited_by {
            crate::user::User::Bot(_) => panic!(),
            crate::user::User::Person(person) => {
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
    }
}
