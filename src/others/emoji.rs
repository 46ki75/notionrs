use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/emoji-object>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Emoji {
    /// The constant string "emoji" that represents the object type.
    pub r#type: String,

    /// The emoji character. For example, "😻".
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

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.emoji)
    }
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
    fn test_emoji() {
        let emoji = Emoji::default();
        assert_eq!(emoji.r#type, "emoji");
        assert_eq!(emoji.emoji, '💡');
        assert_eq!(emoji.to_string(), "💡");

        let emoji = Emoji::from('🔥');
        assert_eq!(emoji.r#type, "emoji");
        assert_eq!(emoji.emoji, '🔥');
        assert_eq!(emoji.to_string(), "🔥");
    }
}
