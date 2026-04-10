use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#paragraph>
///
/// Paragraph block objects contain the following
/// information within the paragraph property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ParagraphBlock {
    /// The rich text displayed in the paragraph block.
    pub rich_text: Vec<crate::object::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::object::color::Color,

    /// An emoji, file, or icon object that represents the paragraph's icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::object::emoji_and_icon::EmojiAndIcon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl ParagraphBlock {
    crate::color_setters!(self, self.color);
}

impl<T> From<T> for ParagraphBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::object::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
    }
}

impl std::fmt::Display for ParagraphBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rich_text
                .iter()
                .map(|t| { t.to_string() })
                .collect::<String>()
        )
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::ParagraphBlock;

    #[test]
    fn deserialize_block_paragraph() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "List Item 1",
                        "link": null
                    },
                    "annotations": {
                        "bold": false,
                        "italic": false,
                        "strikethrough": false,
                        "underline": false,
                        "code": false,
                        "color": "default"
                    },
                    "plain_text": "p",
                    "href": null
                }
            ],
            "color": "default"
        }
        "#;

        let paragraph: ParagraphBlock = serde_json::from_str::<ParagraphBlock>(json_data).unwrap();

        assert_eq!(paragraph.color, crate::object::color::Color::Default);

        let rich_text = paragraph.rich_text.first().unwrap();

        // assert_eq!(rich_text.plain_text, "p");
        // assert_eq!(rich_text.href, None);

        // assert!(!rich_text.annotations.bold);
        // assert!(!rich_text.annotations.italic);
        // assert!(!rich_text.annotations.strikethrough);
        // assert!(!rich_text.annotations.underline);
        // assert!(!rich_text.annotations.code);
        // assert_eq!(
        //     rich_text.annotations.color,
        //     crate::object::color::Color::Default
        // );

        match rich_text {
            crate::object::rich_text::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "p");
                assert_eq!(*href, None);

                assert!(!annotations.bold);
                assert!(!annotations.code);
                assert!(!annotations.strikethrough);
                assert!(!annotations.underline);
                assert!(!annotations.italic);
                assert_eq!(annotations.color, crate::object::color::Color::Default)
            }
            _ => panic!(),
        }
    }

    #[test]
    fn deserialize_block_paragraph_with_icon() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "Tab content",
                        "link": null
                    },
                    "annotations": {
                        "bold": false,
                        "italic": false,
                        "strikethrough": false,
                        "underline": false,
                        "code": false,
                        "color": "default"
                    },
                    "plain_text": "Tab content",
                    "href": null
                }
            ],
            "color": "default",
            "icon": {
                "type": "emoji",
                "emoji": "📝"
            }
        }
        "#;

        let paragraph: ParagraphBlock = serde_json::from_str::<ParagraphBlock>(json_data).unwrap();

        assert_eq!(paragraph.color, crate::object::color::Color::Default);
        assert!(paragraph.icon.is_some());

        match paragraph.icon.unwrap() {
            crate::object::emoji_and_icon::EmojiAndIcon::Emoji(emoji) => {
                assert_eq!(emoji.r#type, "emoji");
                assert_eq!(emoji.emoji, "📝".to_string());
            }
            _ => panic!("Expected emoji icon"),
        }
    }

    #[test]
    fn deserialize_block_paragraph_with_null_icon() {
        let json_data = r#"
        {
            "rich_text": [],
            "color": "default",
            "icon": null
        }
        "#;

        let paragraph: ParagraphBlock = serde_json::from_str::<ParagraphBlock>(json_data).unwrap();
        assert!(paragraph.icon.is_none());
    }

    #[test]
    fn deserialize_block_paragraph_without_icon() {
        let json_data = r#"
        {
            "rich_text": [],
            "color": "default"
        }
        "#;

        let paragraph: ParagraphBlock = serde_json::from_str::<ParagraphBlock>(json_data).unwrap();
        assert!(paragraph.icon.is_none());
    }

    #[test]
    fn serialize_block_paragraph_with_icon() {
        let paragraph = ParagraphBlock {
            rich_text: vec![],
            color: crate::object::color::Color::Default,
            icon: Some(crate::object::emoji_and_icon::EmojiAndIcon::Emoji(
                crate::object::emoji::Emoji {
                    r#type: "emoji".to_string(),
                    emoji: "📝".to_string(),
                },
            )),
            children: None,
        };

        let json = serde_json::to_string(&paragraph).unwrap();
        assert!(json.contains("\"icon\""));
        assert!(json.contains("📝"));
    }

    #[test]
    fn serialize_block_paragraph_without_icon() {
        let paragraph = ParagraphBlock {
            rich_text: vec![],
            color: crate::object::color::Color::Default,
            icon: None,
            children: None,
        };

        let json = serde_json::to_string(&paragraph).unwrap();
        assert!(!json.contains("\"icon\""));
    }
}
