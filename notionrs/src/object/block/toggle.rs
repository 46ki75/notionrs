use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#toggle-blocks>
///
/// Toggle block objects contain the following information within the toggle property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ToggleBlock {
    /// The rich text displayed in the Toggle block.
    pub rich_text: Vec<crate::object::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::object::color::Color,

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl ToggleBlock {
    crate::color_setters!(self, self.color);
}

impl<T> From<T> for ToggleBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::object::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
    }
}

impl std::fmt::Display for ToggleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rich_text
                .iter()
                .map(|t| t.to_string())
                .collect::<String>(),
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

    use super::ToggleBlock;

    #[test]
    fn deserialize_block_toggle() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "summary",
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
                    "plain_text": "summary",
                    "href": null
                }
            ],
            "color": "default"
        }
        "#;

        let toggle = serde_json::from_str::<ToggleBlock>(json_data).unwrap();

        let rich_text = toggle.rich_text.first().unwrap();

        match rich_text {
            crate::object::rich_text::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "summary");
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
}
