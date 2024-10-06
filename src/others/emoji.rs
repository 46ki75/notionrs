use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/emoji-object>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Emoji {
    pub r#type: String,
    pub emoji: char,
}

impl Emoji {
    pub fn new(emoji: char) -> Self {
        Emoji {
            r#type: "emoji".to_string(),
            emoji,
        }
    }
}

impl Default for Emoji {
    fn default() -> Self {
        Emoji::new('💡')
    }
}
