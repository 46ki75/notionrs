use serde::{Deserialize, Serialize};

use crate::color_setters;

/// <https://developers.notion.com/reference/block#numbered-list-item>
///
/// Numbered list item block objects contain the following
/// information within the numbered_list_item property:
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct NumberedListItemBlock {
    /// The rich text displayed in the numbered_list_item block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl NumberedListItemBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn children(mut self, children: Vec<super::Block>) -> Self {
        self.children = Some(children);
        self
    }

    color_setters!(self, self.color);
}

impl<T> From<T> for NumberedListItemBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::others::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
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
            crate::others::color::Color::Default
        );

        let rich_text = numbered_list_item.rich_text.first().unwrap();

        // assert_eq!(rich_text.plain_text, "List Item 1");
        // assert_eq!(rich_text.href, None);

        // assert!(!rich_text.annotations.bold);
        // assert!(!rich_text.annotations.italic);
        // assert!(!rich_text.annotations.strikethrough);
        // assert!(!rich_text.annotations.underline);
        // assert!(!rich_text.annotations.code);
        // assert_eq!(
        //     rich_text.annotations.color,
        //     crate::others::color::Color::Default
        // );

        match rich_text {
            crate::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "List Item 1");
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
    }
}
