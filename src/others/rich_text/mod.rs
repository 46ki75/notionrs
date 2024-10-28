use serde::{Deserialize, Serialize};

use super::color::Color;

pub mod equation;
pub mod mention;
pub mod text;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichText {
    Text {
        text: text::Text,
        annotations: RichTextAnnotations,
        plain_text: String,
        href: Option<String>,
    },
    Mention {
        mention: mention::Mention,
        annotations: RichTextAnnotations,
        plain_text: String,
        href: Option<String>,
    },
    Equation {
        equation: equation::Equation,
        annotations: RichTextAnnotations,
        plain_text: String,
        href: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
pub struct RichTextAnnotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: Color,
}

impl RichText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn plain_text<T>(self, plain_text: T) -> Self
    where
        T: AsRef<str>,
    {
        let plain_text = plain_text.as_ref().to_string();

        match self {
            Self::Text {
                mut text,
                annotations,
                href,
                ..
            } => {
                text.content = plain_text.clone();
                Self::Text {
                    text,
                    annotations,
                    plain_text,
                    href,
                }
            }
            Self::Mention {
                mention,
                annotations,
                href,
                ..
            } => Self::Mention {
                mention,
                annotations,
                plain_text,
                href,
            },
            Self::Equation {
                equation,
                annotations,
                href,
                ..
            } => Self::Equation {
                equation,
                annotations,
                plain_text,
                href,
            },
        }
    }

    pub fn href<T>(self, href: T) -> Self
    where
        T: AsRef<str>,
    {
        let href = Some(href.as_ref().to_string());

        match self {
            Self::Text {
                text,
                annotations,
                plain_text,
                ..
            } => Self::Text {
                text: text.link(href.clone().unwrap_or_default()),
                annotations,
                plain_text,
                href,
            },
            Self::Mention {
                mention,
                annotations,
                plain_text,
                ..
            } => Self::Mention {
                mention,
                annotations,
                plain_text,
                href,
            },
            Self::Equation {
                equation,
                annotations,
                plain_text,
                ..
            } => Self::Equation {
                equation,
                annotations,
                plain_text,
                href,
            },
        }
    }

    pub fn annotations(self, new_annotations: RichTextAnnotations) -> Self {
        match self {
            Self::Text {
                text,
                plain_text,
                href,
                ..
            } => Self::Text {
                text,
                annotations: new_annotations,
                plain_text,
                href,
            },
            Self::Mention {
                mention,
                plain_text,
                href,
                ..
            } => Self::Mention {
                mention,
                annotations: new_annotations,
                plain_text,
                href,
            },
            Self::Equation {
                equation,
                plain_text,
                href,
                ..
            } => Self::Equation {
                equation,
                annotations: new_annotations,
                plain_text,
                href,
            },
        }
    }

    pub fn bold(self) -> Self {
        self.set_annotation(|mut annotations| {
            annotations.bold = true;
            annotations
        })
    }

    pub fn italic(self) -> Self {
        self.set_annotation(|mut annotations| {
            annotations.italic = true;
            annotations
        })
    }

    pub fn strikethrough(self) -> Self {
        self.set_annotation(|mut annotations| {
            annotations.strikethrough = true;
            annotations
        })
    }

    pub fn underline(self) -> Self {
        self.set_annotation(|mut annotations| {
            annotations.underline = true;
            annotations
        })
    }

    pub fn code(self) -> Self {
        self.set_annotation(|mut annotations| {
            annotations.code = true;
            annotations
        })
    }

    pub fn color(self, color: Color) -> Self {
        self.set_annotation(|mut annotations| {
            annotations.color = color;
            annotations
        })
    }

    fn set_annotation<F>(self, update: F) -> Self
    where
        F: FnOnce(RichTextAnnotations) -> RichTextAnnotations,
    {
        let new_annotations = match self {
            Self::Text { annotations, .. } => update(annotations),
            Self::Mention { annotations, .. } => update(annotations),
            Self::Equation { annotations, .. } => update(annotations),
        };

        self.annotations(new_annotations)
    }
}

impl Default for RichText {
    fn default() -> Self {
        Self::Text {
            text: text::Text::default(),
            annotations: RichTextAnnotations::default(),
            plain_text: String::new(),
            href: None,
        }
    }
}

impl std::fmt::Display for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Text { plain_text, .. } => write!(f, "{}", plain_text),
            Self::Mention { plain_text, .. } => write!(f, "{}", plain_text),
            Self::Equation { plain_text, .. } => write!(f, "{}", plain_text),
        }
    }
}

impl<T> From<T> for RichText
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let plain_text = value.as_ref().to_string();
        Self::Text {
            text: text::Text {
                content: plain_text.clone(),
                ..Default::default()
            },
            plain_text,
            annotations: RichTextAnnotations::default(),
            href: None,
        }
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use core::panic;

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

        match rich_text {
            RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "rich text");
                assert_eq!(href, None);

                assert!(!annotations.bold);
                assert!(!annotations.italic);
                assert!(!annotations.strikethrough);
                assert!(!annotations.underline);
                assert!(!annotations.code);
                assert_eq!(annotations.color, crate::others::color::Color::Default);
            }
            _ => panic!("Unexpected variant!"),
        }
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
