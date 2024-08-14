pub mod bot;
pub mod person;

use serde::{Deserialize, Serialize};

use self::{bot::Bot, person::Person};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum User {
    Bot(Bot),
    Person(Person),
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
    fn unit_test_deserialize_user_bot() {
        let json_data = r#"
        {
            "object": "user",
            "id": "015a538b-bc75-4327-8b89-8847bf01705a",
            "name": "notionrs-integration-test",
            "avatar_url": null,
            "type": "bot",
            "bot": {
                "owner": {
                    "type": "workspace",
                    "workspace": true
                },
                "workspace_name": "notionrs integration test"
            }
        }
        "#;

        let user = serde_json::from_str::<User>(json_data).unwrap();

        match user {
            User::Bot(bot) => {
                assert_eq!(bot.object, "user".to_string());
                assert_eq!(bot.id, "015a538b-bc75-4327-8b89-8847bf01705a".to_string());
                assert_eq!(bot.name, Some("notionrs-integration-test".to_string()));
                assert_eq!(bot.avatar_url, None);
                assert_eq!(bot.r#type, Some("bot".to_string()));
            }
            User::Person(_) => {
                panic!("The expected enum during deserialization was Bot, but it was deserialized as Person.");
            }
        }
    }

    #[test]
    fn unit_test_deserialize_user_person() {
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

        let user = serde_json::from_str::<User>(json_data).unwrap();

        match user {
            User::Bot(_) => {
                panic!("The expected enum during deserialization was Person, but it was deserialized as Bot.");
            }
            User::Person(user) => {
                assert_eq!(user.object, "user".to_string());
                assert_eq!(user.id, "78126152-3c2a-4b04-860e-77fb5bdded2f".to_string());
                assert_eq!(user.name, Some("John Doe".to_string()));
                assert_eq!(
                    user.avatar_url,
                    Some("https://example.com/avatar.png".to_string())
                );
                assert_eq!(user.r#type, Some("person".to_string()));

                match user.person {
                    Some(person) => {
                        assert_eq!(person.email, Some("johndoe@example.com".to_string()))
                    }
                    None => {
                        panic!("The 'email' should exist, but it was not found.");
                    }
                }
            }
        }
    }
}
