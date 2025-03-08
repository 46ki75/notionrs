use core::fmt;

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

impl fmt::Display for CustomEmoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.custom_emoji.name)
    }
}

impl fmt::Display for CustomEmojiContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
