use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#code>
///
/// Code block objects contain the following
/// information within the code property:
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CodeBlock {
    /// The rich text in the caption of the code block.
    pub caption: Vec<crate::others::rich_text::RichText>,

    /// The rich text in the code block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The language of the code contained in the code block.
    pub language: crate::others::language::Language,
}

impl CodeBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        self.caption = caption;
        self
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn lnaguage(mut self, language: crate::others::language::Language) -> Self {
        self.language = language;
        self
    }
}

impl<T> From<T> for CodeBlock
where
    T: AsRef<str>,
{
    fn from(code: T) -> Self {
        let rich_text = crate::others::rich_text::RichText::from(code.as_ref().to_string());
        Self::new().rich_text(vec![rich_text])
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::CodeBlock;

    #[test]
    fn deserialize_block_code() {
        let json_data = r#"
        {
            "caption": [
                {
                    "type": "text",
                    "text": {
                        "content": "src/others/emoji.rs",
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
                    "plain_text": "src/others/emoji.rs",
                    "href": null
                }
            ],
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Deserialize, Serialize)]\npub struct Emoji {\n    pub r#type: String,\n    pub emoji: String,\n}",
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
                    "plain_text": "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Deserialize, Serialize)]\npub struct Emoji {\n    pub r#type: String,\n    pub emoji: String,\n}",
                    "href": null
                }
            ],
            "language": "rust"
        }
        "#;

        let code = serde_json::from_str::<CodeBlock>(json_data).unwrap();

        assert_eq!(code.language, crate::others::language::Language::Rust);

        let caption = code.caption.first().unwrap();

        match caption {
            crate::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "src/others/emoji.rs");
                assert_eq!(*href, None);

                assert!(!annotations.bold);
                assert!(!annotations.code);
                assert!(!annotations.strikethrough);
                assert!(!annotations.underline);
                assert!(!annotations.italic);
                assert_eq!(annotations.color, crate::others::color::Color::Default)
            }
            _ => panic!(),
        }

        let rich_text = code.rich_text.first().unwrap();

        match rich_text {
            crate::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert!(plain_text.contains("use serde::{Deserialize, Serialize};"));
                assert_eq!(*href, None);

                assert!(!annotations.bold);
                assert!(!annotations.code);
                assert!(!annotations.strikethrough);
                assert!(!annotations.underline);
                assert!(!annotations.italic);
                assert_eq!(annotations.color, crate::others::color::Color::Default)
            }
            _ => panic!(),
        }
    }
}
