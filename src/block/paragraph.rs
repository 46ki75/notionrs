use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#paragraph>
///
/// Paragraph block objects contain the following
/// information within the paragraph property:
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ParagraphBlock {
    /// The rich text displayed in the paragraph block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

impl ParagraphBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    crate::color_setters!(self, self.color);
}

impl<T> From<T> for ParagraphBlock
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

    use super::ParagraphBlock;

    #[test]
    fn deserialize_block_paragraph() {
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
                    "plain_text": "p",
                    "href": null
                }
            ],
            "color": "default"
        }
        "#;

        let paragraph: ParagraphBlock = serde_json::from_str::<ParagraphBlock>(json_data).unwrap();

        assert_eq!(paragraph.color, crate::others::color::Color::Default);

        let rich_text = paragraph.rich_text.first().unwrap();

        // assert_eq!(rich_text.plain_text, "p");
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
                assert_eq!(plain_text, "p");
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
