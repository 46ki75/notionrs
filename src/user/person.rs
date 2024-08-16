use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/user#people>
/// User objects that represent people have the type property set to "person".
#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    /// always "user"
    pub object: String,

    /// Unique identifier for this user.
    pub id: String,

    /// User's name, as displayed in Notion.
    pub name: Option<String>,

    /// Chosen avatar image.
    pub avatar_url: Option<String>,

    /// always "person"
    pub r#type: Option<String>,

    /// Properties only present for non-bot users.
    pub person: Option<PersonDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDetail {
    /// Email address of person. This is only present if an integration has
    /// user capabilities that allow access to email addresses.
    pub email: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_person() {
        let json_data = r#"
        {
            "object": "user",
            "id": "78126152-3c2a-4b04-860e-77fb5bdded2f",
            "name": "John Doe",
            "avatar_url": "https://example.com/avatar.png",
            "type": "person",
            "person": {
                "email": "johndoe@example.com"
            }
        }
        "#;

        let person: Person = serde_json::from_str(json_data).unwrap();

        assert_eq!(person.object, "user");
        assert_eq!(person.id, "78126152-3c2a-4b04-860e-77fb5bdded2f");
        assert_eq!(person.name.unwrap(), "John Doe");
        assert_eq!(person.avatar_url.unwrap(), "https://example.com/avatar.png");
        assert_eq!(person.r#type.unwrap(), "person");
        assert_eq!(person.person.unwrap().email.unwrap(), "johndoe@example.com");
    }
}
