use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum EmojiAndIcon {
    File(crate::object::file::File),
    Emoji(crate::object::emoji::Emoji),
    CustomEmoji(crate::object::emoji::CustomEmoji),
    Icon(crate::object::icon::Icon),
}

impl Default for EmojiAndIcon {
    fn default() -> Self {
        EmojiAndIcon::Emoji(crate::object::emoji::Emoji::default())
    }
}

impl std::fmt::Display for EmojiAndIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmojiAndIcon::File(file) => write!(f, "{}", file),
            EmojiAndIcon::Emoji(emoji) => write!(f, "{}", emoji),
            EmojiAndIcon::CustomEmoji(custom_emoji) => write!(f, "{}", custom_emoji),
            EmojiAndIcon::Icon(icon) => write!(f, "{}", icon),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_emoji() {
        let emoji = r#"
        {
            "type": "emoji",
            "emoji": "😀"
        }
        "#;

        let emoji: EmojiAndIcon = serde_json::from_str(emoji).unwrap();
        assert!(matches!(emoji, EmojiAndIcon::Emoji(_)));
    }

    #[test]
    fn icon_file() {
        let file = r#"
        {
            "type": "file",
            "file": {
                "url": "https://www.notion.so/image.png",
                "expiry_time": "2022-09-01T00:00:00Z"
            }
        }
        "#;

        let file: EmojiAndIcon = serde_json::from_str(file).unwrap();
        assert!(matches!(file, EmojiAndIcon::File(_)));
    }

    #[test]
    fn icon_custom_emoji() {
        let custom_emoji = r#"
        {
            "type": "emoji",
            "custom_emoji": {
                "id": "123",
                "name": "custom_emoji",
                "url": "https://www.notion.so/image.png"
            }
        }
        "#;

        let custom_emoji: EmojiAndIcon = serde_json::from_str(custom_emoji).unwrap();
        assert!(matches!(custom_emoji, EmojiAndIcon::CustomEmoji(_)));
    }
}
