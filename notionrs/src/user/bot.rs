use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/user#bots>
/// A user object's type property is"bot" when the user object represents a bot.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Bot {
    /// always "user"
    pub object: String,

    /// bot identifier
    pub id: String,

    /// Integration name.
    /// You can check it [here](https://www.notion.so/profile/integrations).
    pub name: Option<String>,

    /// Within the Notion API Integration page, when you upload an image,
    /// its storage URL is captured and stored in this field.
    pub avatar_url: Option<String>,

    /// always "bot"
    pub r#type: Option<String>,

    /// Since all fields are optional, it might result in an empty object. `{}`
    pub bot: BotDetail,
}

/// This struct can potentially become an empty object since all its fields are optional.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BotDetail {
    /// Information about who owns this bot.
    pub owner: Option<BotOwner>,

    /// If the owner.type is "workspace", then workspace.name identifies
    /// the name of the workspace that owns the bot.
    /// If the owner.type is "user", then workspace.name is null.
    pub workspace_name: Option<String>,
}

/// Information about who owns this bot.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BotOwner {
    /// The type of owner, either "workspace" or "user".
    pub r#type: String,

    /// Whether the bot's owner is the workspace.
    pub workspace: bool,
}

impl std::fmt::Display for Bot {
    /// Display the name if it exists, otherwise display the id.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)
        } else {
            write!(f, "{}", self.id)
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
    use serde_json;

    #[test]
    fn deserialize_bot() {
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

        let bot: Bot = serde_json::from_str(json_data).unwrap();

        assert_eq!(bot.object, "user");
        assert_eq!(bot.id, "015a538b-bc75-4327-8b89-8847bf01705a");
        assert_eq!(bot.name, Some("notionrs-integration-test".to_string()));
        assert_eq!(bot.avatar_url, None);
        assert_eq!(
            bot.bot.workspace_name,
            Some("notionrs integration test".to_string())
        );
    }
}
