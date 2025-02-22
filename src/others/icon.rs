use serde::{Deserialize, Serialize};

use super::{custom_emoji::CustomEmoji, emoji::Emoji, file::File};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
    CustomEmoji(CustomEmoji),
}

impl Default for Icon {
    fn default() -> Self {
        Icon::Emoji(Emoji::default())
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Icon::File(file) => write!(f, "{}", file),
            Icon::Emoji(emoji) => write!(f, "{}", emoji),
            Icon::CustomEmoji(custom_emoji) => write!(f, "{}", custom_emoji),
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

        let emoji: Icon = serde_json::from_str(emoji).unwrap();
        assert!(matches!(emoji, Icon::Emoji(_)));
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

        let file: Icon = serde_json::from_str(file).unwrap();
        assert!(matches!(file, Icon::File(_)));
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

        let custom_emoji: Icon = serde_json::from_str(custom_emoji).unwrap();
        assert!(matches!(custom_emoji, Icon::CustomEmoji(_)));
    }
}
