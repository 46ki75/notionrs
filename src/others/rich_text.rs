use serde::{Deserialize, Serialize};

use crate::color_setters;

use super::color::Color;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RichText {
    pub r#type: String,
    pub text: RichTextContent,
    pub annotations: RichTextAnnotations,
    pub plain_text: String,
    pub href: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RichTextContent {
    pub content: String,
    pub link: Option<RichTextLink>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct RichTextAnnotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: Color,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RichTextLink {
    pub url: String,
}

impl RichText {
    pub fn new() -> Self {
        RichText {
            r#type: "text".to_string(),
            text: RichTextContent {
                content: String::new(),
                link: None,
            },
            annotations: RichTextAnnotations {
                bold: false,
                italic: false,
                strikethrough: false,
                underline: false,
                code: false,
                color: crate::others::color::Color::Default,
            },
            plain_text: String::new(),
            href: None,
        }
    }

    pub fn href<T>(mut self, href: T) -> Self
    where
        T: AsRef<str>,
    {
        self.href = Some(href.as_ref().to_string());
        self.text.link = Some(RichTextLink {
            url: href.as_ref().to_string(),
        });
        self
    }

    pub fn plain_text<T>(mut self, plain_text: T) -> Self
    where
        T: AsRef<str>,
    {
        self.text.content = plain_text.as_ref().to_string();
        self.plain_text = plain_text.as_ref().to_string();
        self
    }

    pub fn color(mut self, color: crate::others::color::Color) -> Self {
        self.annotations.color = color;
        self
    }

    color_setters!(self, self.annotations.color);

    pub fn bold(mut self) -> Self {
        self.annotations.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.annotations.italic = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.annotations.strikethrough = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.annotations.underline = true;
        self
    }

    pub fn code(mut self) -> Self {
        self.annotations.code = true;
        self
    }
}

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for RichText
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        Self::new().plain_text(plain_text.as_ref())
    }
}

impl std::fmt::Display for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.plain_text)
    }
}

impl From<RichText> for String {
    fn from(rich_text: RichText) -> Self {
        rich_text.to_string()
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn deserialize_rich_text() {
        let json_data = r#"
        {
            "type": "text",
            "text": {
                "content": "rich text",
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
            "plain_text": "rich text",
            "href": null
        }
        "#;

        let rich_text = serde_json::from_str::<RichText>(json_data).unwrap();

        assert_eq!(rich_text.plain_text, "rich text");
        assert_eq!(rich_text.href, None);

        assert!(!rich_text.annotations.bold);
        assert!(!rich_text.annotations.italic);
        assert!(!rich_text.annotations.strikethrough);
        assert!(!rich_text.annotations.underline);
        assert!(!rich_text.annotations.code);
        assert_eq!(
            rich_text.annotations.color,
            crate::others::color::Color::Default
        );
    }

    #[test]
    fn serialize_rich_text() {
        let rich_text = RichText::new()
            .plain_text("My Text")
            .bold()
            .italic()
            .strikethrough()
            .code()
            .color(crate::others::color::Color::Red)
            .href("https://example.com");

        let expected_json = r#"
        {
            "type": "text",
            "text": {
                "content": "My Text",
                "link": {
                    "url": "https://example.com"
                }
            },
            "annotations": {
                "bold": true,
                "italic": true,
                "strikethrough": true,
                "underline": false,
                "code": true,
                "color": "red"
            },
            "plain_text": "My Text",
            "href": "https://example.com"
        }
        "#;

        let serialized = serde_json::to_string_pretty(&rich_text).unwrap();

        let expected_value: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        let serialized_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        assert_eq!(serialized_value, expected_value);
    }
}
