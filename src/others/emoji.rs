use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/emoji-object>
#[derive(Debug, Deserialize, Serialize)]
pub struct Emoji {
    pub r#type: String,
    pub emoji: String,
}

impl Emoji {
    pub fn new<T>(emoji: T) -> Self
    where
        T: AsRef<str>,
    {
        Emoji {
            r#type: "emoji".to_string(),
            emoji: emoji.as_ref().to_string(),
        }
    }
}

impl Default for Emoji {
    fn default() -> Self {
        Emoji::new("💡")
    }
}
