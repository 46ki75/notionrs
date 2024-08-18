use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#quote>
///
/// Paragraph block objects contain the following
/// information within the quote property:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QuoteBlock {
    /// The rich text displayed in the quote block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

impl QuoteBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Quote { quote: self }
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

    use super::QuoteBlock;

    #[test]
    fn deserialize_block_quote() {
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
                    "plain_text": "quote",
                    "href": null
                }
            ],
            "color": "default"
        }
        "#;

        let quote: QuoteBlock = serde_json::from_str::<QuoteBlock>(json_data).unwrap();

        assert_eq!(
            quote.color,
            crate::others::color::Color::FG(crate::others::color::ColorFG::Default)
        );

        let rich_text = quote.rich_text.first().unwrap();

        assert_eq!(rich_text.plain_text, "quote");
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
