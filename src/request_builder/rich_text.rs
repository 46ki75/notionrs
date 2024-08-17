use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RichTextRequest {
    pub r#type: String,
    pub text: RichTextContentRequest,
    pub annotations: Option<RichTextAnnotationsRequest>,
    pub plain_text: Option<String>,
    pub href: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RichTextContentRequest {
    pub content: String,
    pub link: Option<RichTextLinkRequest>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
pub struct RichTextAnnotationsRequest {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<bool>,
    pub code: Option<bool>,
    pub color: Option<crate::others::color::Color>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RichTextLinkRequest {
    pub url: String,
}

impl RichTextRequest {
    pub fn new<T>(plain_text: T) -> Self
    where
        T: AsRef<str>,
    {
        RichTextRequest {
            r#type: "text".to_string(),
            text: RichTextContentRequest {
                content: plain_text.as_ref().to_string(),
                link: None,
            },
            annotations: None,
            plain_text: None,
            href: None,
        }
    }

    pub fn link<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.text.link = Some(RichTextLinkRequest {
            url: url.as_ref().to_string(),
        });
        self
    }

    pub fn bold(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.bold = Some(true);
        self.annotations = Some(annotations);
        self
    }

    pub fn italic(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.italic = Some(true);
        self.annotations = Some(annotations);
        self
    }

    pub fn strikethrough(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.strikethrough = Some(true);
        self.annotations = Some(annotations);
        self
    }

    pub fn underline(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.underline = Some(true);
        self.annotations = Some(annotations);
        self
    }

    pub fn code(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.code = Some(true);
        self.annotations = Some(annotations);
        self
    }

    pub fn default_color(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Default,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn blue(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Blue,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn brown(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Brown,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn gray(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Gray,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn green(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Green,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn orange(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Orange,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn pink(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Pink,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn purple(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Purple,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn red(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Red,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn yellow(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::FG(
            crate::others::color::ColorFG::Yellow,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn blue_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::BlueBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn brown_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::BrownBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn gray_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::GrayBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn green_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::GreenBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn orange_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::OrangeBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn pink_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::PinkBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn purple_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::PurpleBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn red_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::RedBackground,
        ));
        self.annotations = Some(annotations);
        self
    }

    pub fn yellow_background(mut self) -> Self {
        let mut annotations = self.annotations.unwrap_or_default();
        annotations.color = Some(crate::others::color::Color::BG(
            crate::others::color::ColorBG::YellowBackground,
        ));
        self.annotations = Some(annotations);
        self
    }
}
