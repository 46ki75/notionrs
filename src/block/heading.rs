use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#headings>
///
/// All heading block objects, heading_1, heading_2, and heading_3,
/// contain the following information within their corresponding objects:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct HeadingBlock {
    /// The rich text of the heading.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,

    /// Whether or not the heading block is a toggle heading or not.
    /// If true, then the heading block toggles and can support children.
    /// If false, then the heading block is a static heading block.
    pub is_toggleable: bool,
}

impl HeadingBlock {
    pub fn build_heading_1(self) -> super::BlockType {
        super::BlockType::Heading1 { heading_1: self }
    }

    pub fn build_heading_2(self) -> super::BlockType {
        super::BlockType::Heading2 { heading_2: self }
    }

    pub fn build_heading_3(self) -> super::BlockType {
        super::BlockType::Heading3 { heading_3: self }
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

    use super::HeadingBlock;

    #[test]
    fn deserialize_block_heading() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "Heading 1",
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
                    "plain_text": "Heading",
                    "href": null
                }
            ],
            "is_toggleable": false,
            "color": "default"
        }
        "#;

        let heading: HeadingBlock = serde_json::from_str::<HeadingBlock>(json_data).unwrap();

        assert_eq!(
            heading.color,
            crate::others::color::Color::FG(crate::others::color::ColorFG::Default)
        );

        assert!(!heading.is_toggleable);

        let rich_text = heading.rich_text.first().unwrap();

        assert_eq!(rich_text.plain_text, "Heading");
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
