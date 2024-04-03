use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(Debug, Deserialize, Serialize)]
pub struct RichText {
    pub r#type: String,
    pub text: RichTextContent,
    pub annotations: RichTextAnnotations,
    pub plain_text: String,
    pub href: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RichTextContent {
    pub content: String,
    pub link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RichTextAnnotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: Color,
}
