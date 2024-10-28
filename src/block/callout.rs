use serde::{Deserialize, Serialize};

use crate::color_setters;

/// <https://developers.notion.com/reference/block#callout>
///
/// Callout block objects contain the following
/// information within the callout property:
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CalloutBlock {
    /// The rich text in the callout block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// An emoji or file object that represents the callout's icon. If the callout does not have an icon.
    pub icon: crate::others::icon::Icon,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

impl CalloutBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn icon_emoji(mut self, emoji: char) -> Self {
        self.icon = crate::others::icon::Icon::new_emoji(emoji);
        self
    }

    pub fn icon_file<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut icon_file = crate::others::icon::Icon::new_file();
        if let crate::others::icon::Icon::File(crate::others::file::File::External(
            ref mut external,
        )) = icon_file
        {
            external.url(url);
        }
        self.icon = icon_file;
        self
    }

    color_setters!(self, self.color);
}

impl<T> From<T> for CalloutBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::others::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
    }
}

impl std::fmt::Display for CalloutBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rich_text
                .iter()
                .map(|t| t.to_string())
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

    use super::CalloutBlock;

    #[test]
    fn deserialize_block_callout() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "My Callout",
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
                    "plain_text": "My Callout",
                    "href": null
                }
            ],
            "icon": {
                "type": "emoji",
                "emoji": "💡"
            },
            "color": "blue_background"
        }
        "#;

        let callout: CalloutBlock = serde_json::from_str::<CalloutBlock>(json_data).unwrap();

        assert_eq!(callout.color, crate::others::color::Color::BlueBackground);

        let rich_text = callout.rich_text.first().unwrap();

        match rich_text {
            crate::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "My Callout");
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

        match callout.icon {
            crate::others::icon::Icon::Emoji(emoji) => {
                assert_eq!(emoji.r#type, "emoji");
                assert_eq!(emoji.emoji, '💡');
            }
            _ => panic!("Unexpected!"),
        };
    }
}
