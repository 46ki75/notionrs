use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/emoji-object>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Emoji {
    /// The constant string "emoji" that represents the object type.
    pub r#type: String,

    /// The emoji character. For example, "😻".
    pub emoji: String,
}

impl Emoji {
    pub fn emoji(mut self, emoji: String) -> Self {
        self.emoji = emoji;
        self
    }
}

impl Default for Emoji {
    /// Create a new Emoji.
    /// default emoji: 💡
    fn default() -> Self {
        Emoji {
            r#type: "emoji".to_string(),
            emoji: '💡'.to_string(),
        }
    }
}

impl<T> From<T> for Emoji
where
    T: AsRef<str>,
{
    /// Convert from a char to an Emoji.
    fn from(value: T) -> Self {
        Self::default().emoji(value.as_ref().to_string())
    }
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.emoji)
    }
}

/// <https://developers.notion.com/reference/emoji-object#custom-emoji>
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct CustomEmoji {
    /// The constant string "custom_emoji" that represents the object type.
    pub r#type: String,

    /// Custom emoji object, containing id, name, url
    pub custom_emoji: CustomEmojiContent,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct CustomEmojiContent {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl std::fmt::Display for CustomEmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.custom_emoji.name)
    }
}

impl std::fmt::Display for CustomEmojiContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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
    fn test_emoji_default() {
        let emoji = Emoji::default();
        assert_eq!(emoji.r#type, "emoji");
        assert_eq!(emoji.emoji, "💡".to_string());
        assert_eq!(emoji.to_string(), "💡");
    }

    #[test]
    fn test_emoji_from() {
        let emoji = Emoji::from("🔥");
        assert_eq!(emoji.r#type, "emoji");
        assert_eq!(emoji.emoji, "🔥".to_string());
        assert_eq!(emoji.to_string(), "🔥");
    }

    #[test]
    fn test_emoji_into() {
        let emoji = Emoji::from("🔥");
        assert_eq!(emoji.r#type, "emoji");
        assert_eq!(emoji.emoji, "🔥".to_string());
        assert_eq!(emoji.to_string(), "🔥");
    }

    #[test]
    fn test_emoji_display() {
        let emoji = Emoji::default();
        assert_eq!(emoji.to_string(), "💡");
    }
}
