use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#toggle-blocks>
///
/// Toggle block objects contain the following information within the toggle property:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ToggleBlock {
    /// The rich text displayed in the Toggle block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

impl ToggleBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Toggle { toggle: self }
    }

    pub fn new() -> Self {
        Self::default()
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

        assert_eq!(rich_text.plain_text, "summary");
        assert_eq!(rich_text.href, None);

        assert!(!rich_text.annotations.bold);
        assert!(!rich_text.annotations.italic);
        assert!(!rich_text.annotations.strikethrough);
        assert!(!rich_text.annotations.underline);
        assert!(!rich_text.annotations.code);
        assert_eq!(
            rich_text.annotations.color,
            crate::others::color::Color::FG(crate::others::color::ColorFG::Default)
        );
    }
}
