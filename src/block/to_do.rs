use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#to-do>
///
/// To do block objects contain the following information within the to_do property:
#[derive(Deserialize, Serialize, Debug)]
pub struct ToDoBlock {
    /// The rich text displayed in the To do block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// Whether the To do is checked.
    pub checked: bool,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::ToDoBlock;

    #[test]
    fn deserialize_block_to_do() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "TODO",
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
                    "plain_text": "TODO",
                    "href": null
                }
            ],
            "checked": true,
            "color": "default"
        }
        "#;

        let to_do = serde_json::from_str::<ToDoBlock>(json_data).unwrap();

        let rich_text = to_do.rich_text.first().unwrap();

        assert_eq!(rich_text.plain_text, "TODO");
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
