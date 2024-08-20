use serde::{Deserialize, Serialize};

use crate::color_setters;

/// <https://developers.notion.com/reference/block#bulleted-list-item>
///
///  Bulleted list item block objects contain the following
/// information within the bulleted_list_item property:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BulletedListItemBlock {
    /// The rich text in the bulleted_list_item block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::BlockType>>,
}

impl BulletedListItemBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::BulletedListItem {
            bulleted_list_item: self,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn children(mut self, children: Vec<super::BlockType>) -> Self {
        self.children = Some(children);
        self
    }

    color_setters!(self, self.color);
}

impl From<Vec<crate::others::rich_text::RichText>> for BulletedListItemBlock {
    fn from(rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        Self::new().rich_text(rich_text)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::BulletedListItemBlock;

    #[test]
    fn deserialize_block_bulleted_list_item() {
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

        let bulleted_list_item: BulletedListItemBlock =
            serde_json::from_str::<BulletedListItemBlock>(json_data).unwrap();

        assert_eq!(
            bulleted_list_item.color,
            crate::others::color::Color::FG(crate::others::color::ColorFG::Default)
        );

        let rich_text = bulleted_list_item.rich_text.first().unwrap();

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
