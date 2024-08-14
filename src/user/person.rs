use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    pub object: String, // always "user"
    pub id: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,

    /// always "person"
    pub r#type: Option<String>,
    pub person: Option<PersonDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDetail {
    pub email: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn unit_test_deserialize_person() {
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
