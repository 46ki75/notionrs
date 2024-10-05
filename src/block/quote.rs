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

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl QuoteBlock {
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

    crate::color_setters!(self, self.color);
}

impl<T> From<T> for QuoteBlock
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
