use serde::{Deserialize, Serialize};

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
    pub link: Option<String>,
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

impl RichText {
    pub fn new<T>(plain_text: T) -> Self
    where
        T: AsRef<str>,
    {
        RichText {
            r#type: "rich_text".to_string(),
            text: RichTextContent {
                content: plain_text.as_ref().to_string(),
                link: None,
            },
            annotations: RichTextAnnotations {
                bold: false,
                italic: false,
                strikethrough: false,
                underline: false,
                code: false,
                color: crate::others::color::Color::FG(crate::others::color::ColorFG::Default),
            },
            plain_text: plain_text.as_ref().to_string(),
            href: None,
        }
    }

    pub fn href<T>(mut self, href: T) -> Self
    where
        T: AsRef<str>,
    {
        self.href = Some(href.as_ref().to_string());
        self.text.link = Some(href.as_ref().to_string());
        self
    }

    pub fn color(mut self, color: crate::others::color::Color) -> Self {
        self.annotations.color = color;
        self
    }

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
