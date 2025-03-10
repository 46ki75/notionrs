use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#people>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"people"`
/// - `$.['*'].people`: An array of user objects.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example people page property value
///
/// ```json
/// {
///   "Person": {
///     "type": "people",
///     "id": "pAoV",
///     "people": [
///       {
///         "object": "user",
///         "id": "4050d499-9586-4352-85a2-d4cb55a68200",
///         "name": "46ki75",
///         "avatar_url": null,
///         "type": "person",
///         "person": { "email": "46ki75@example.com" }
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PagePeopleProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An array of user objects.
    pub people: Vec<crate::object::user::User>,
}

impl std::fmt::Display for PagePeopleProperty {
    /// Display user names or user IDs separated by commas
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.people
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
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
    use core::panic;

    use super::*;

    #[test]
    fn deserialize_people_property() {
        let json_data = r#"
        {
            "Person": {
                "type": "people",
                "id": "pAoV",
                "people": [
                    {
                        "object": "user",
                        "id": "4050d499-9586-4352-85a2-d4cb55a68200",
                        "name": "46ki75",
                        "avatar_url": null,
                        "type": "person",
                        "person": {
                            "email": "46ki75@example.com"
                        }
                    }
                ]
            }
        }
        "#;

        let people_map = serde_json::from_str::<
            std::collections::HashMap<String, PagePeopleProperty>,
        >(json_data)
        .unwrap();

        let people = people_map.get("Person").unwrap();

        assert_eq!(people.id, Some("pAoV".to_string()));

        match &people.people.first().unwrap() {
            crate::object::user::User::Person(p) => {
                assert_eq!(p.id, "4050d499-9586-4352-85a2-d4cb55a68200");
                assert_eq!(p.name, Some("46ki75".to_string()));
                assert_eq!(p.avatar_url, None);
                assert_eq!(p.r#type, Some("person".to_string()));
                assert_eq!(
                    p.person.as_ref().unwrap().email,
                    Some("46ki75@example.com".to_string())
                );
            }
            crate::object::user::User::Bot(_) => {
                panic!()
            }
        }
    }
}
