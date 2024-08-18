use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#numbered-list-item>
///
/// Numbered list item block objects contain the following
/// information within the numbered_list_item property:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct NumberedListItemBlock {
    /// The rich text displayed in the numbered_list_item block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

impl NumberedListItemBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::NumberedListItem {
            numbered_list_item: self,
        }
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

    use super::NumberedListItemBlock;

    #[test]
    fn deserialize_block_numbered_list_item() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "List Item 1",
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
                    "plain_text": "List Item 1",
                    "href": null
                }
            ],
            "color": "default"
        }
        "#;

        let numbered_list_item: NumberedListItemBlock =
            serde_json::from_str::<NumberedListItemBlock>(json_data).unwrap();

        assert_eq!(
            numbered_list_item.color,
            crate::others::color::Color::FG(crate::others::color::ColorFG::Default)
        );

        let rich_text = numbered_list_item.rich_text.first().unwrap();

        assert_eq!(rich_text.plain_text, "List Item 1");
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
