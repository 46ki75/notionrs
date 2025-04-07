use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
// #[serde(untagged)]
// pub enum User {
//     Bot(bot::Bot),
//     Person(person::Person),
// }

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct User {
    /// always "user"
    pub object: String,

    /// Unique identifier for this user.
    pub id: String,

    /// User's name, as displayed in Notion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Chosen avatar image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// "person" or "bot"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Properties only present for non-bot users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,

    /// Since all fields are optional, it might result in an empty object. `{}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<BotDetail>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct PersonDetail {
    /// Email address of person. This is only present if an integration has
    /// user capabilities that allow access to email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// This struct can potentially become an empty object since all its fields are optional.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct BotDetail {
    /// Information about who owns this bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<BotOwner>,

    /// If the owner.type is "workspace", then workspace.name identifies
    /// the name of the workspace that owns the bot.
    /// If the owner.type is "user", then workspace.name is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

/// Information about who owns this bot.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct BotOwner {
    /// The type of owner, either "workspace" or "user".
    pub r#type: String,

    /// Whether the bot's owner is the workspace.
    pub workspace: bool,
}

impl Default for User {
    fn default() -> Self {
        Self {
            object: String::from("user"),
            id: String::default(),
            name: None,
            avatar_url: None,
            r#type: None,
            person: None,
            bot: None,
        }
    }
}

crate::impl_from_as_ref!(User, id);
crate::impl_display_from_string_field!(User, id);

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_user_bot() -> Result<(), crate::error::Error> {
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

        let user = serde_json::from_str::<User>(json_data)?;

        assert_eq!(user.object, "user".to_string());
        assert_eq!(user.id, "015a538b-bc75-4327-8b89-8847bf01705a".to_string());
        assert_eq!(user.name, Some("notionrs-integration-test".to_string()));
        assert_eq!(user.avatar_url, None);
        assert_eq!(user.r#type, Some("bot".to_string()));

        Ok(())
    }

    #[test]
    fn deserialize_user_person() -> Result<(), crate::error::Error> {
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

        let user = serde_json::from_str::<User>(json_data)?;

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

        Ok(())
    }
}
