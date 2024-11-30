use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/emoji-object>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Emoji {
    pub r#type: String,
    pub emoji: char,
}

impl Emoji {
    pub fn emoji(mut self, emoji: char) -> Self {
        self.emoji = emoji;
        self
    }
}

impl Default for Emoji {
    fn default() -> Self {
        Emoji {
            r#type: "emoji".to_string(),
            emoji: '💡',
        }
    }
}

impl From<char> for Emoji {
    fn from(value: char) -> Self {
        Self::default().emoji(value)
    }
}
