use serde::{Deserialize, Serialize};

use crate::color_setters;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CalloutBlockRequest {
    r#type: String,
    callout: CalloutBlockRequestParams,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CalloutBlockRequestParams {
    /// The rich text in the callout block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// An emoji or file object that represents the callout's icon. If the callout does not have an icon.
    pub icon: crate::others::icon::Icon,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

impl CalloutBlockRequest {
    pub fn build(self) -> super::BlockRequest {
        super::BlockRequest::Callout(self)
    }

    pub fn new() -> Self {
        CalloutBlockRequest {
            r#type: "callout".to_string(),
            callout: CalloutBlockRequestParams::default(),
        }
    }

    pub fn icon_emoji(mut self, emoji: char) -> Self {
        if let crate::others::icon::Icon::Emoji(ref mut emoji_struct) = self.callout.icon {
            emoji_struct.emoji = emoji;
        }
        self
    }

    pub fn icon_file<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.callout.icon = crate::others::icon::Icon::new_file(url);
        self
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.callout.rich_text = rich_text;
        self
    }

    color_setters!(self, self.callout.color);
}
