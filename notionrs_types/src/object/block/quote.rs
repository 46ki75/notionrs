use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#quote>
///
/// Paragraph block objects contain the following
/// information within the quote property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct QuoteBlock {
    /// The rich text displayed in the quote block.
    pub rich_text: Vec<crate::object::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::object::color::Color,

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl QuoteBlock {
    crate::color_setters!(self, self.color);
}

impl<T> From<T> for QuoteBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::object::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
    }
}

impl std::fmt::Display for QuoteBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rich_text
                .iter()
                .map(|t| { t.to_string() })
                .collect::<String>()
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

        assert_eq!(quote.color, crate::object::color::Color::Default);

        let rich_text = quote.rich_text.first().unwrap();

        // assert_eq!(rich_text.plain_text, "quote");
        // assert_eq!(rich_text.href, None);

        // assert!(!rich_text.annotations.bold);
        // assert!(!rich_text.annotations.italic);
        // assert!(!rich_text.annotations.strikethrough);
        // assert!(!rich_text.annotations.underline);
        // assert!(!rich_text.annotations.code);
        // assert_eq!(
        //     rich_text.annotations.color,
        //     crate::object::color::Color::Default
        // );

        match rich_text {
            crate::object::rich_text::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "quote");
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

    #[test]
    fn quote_color_setters() {
        use crate::object::color::Color;
        let q = QuoteBlock::default();
        assert_eq!(q.clone().default_color().color, Color::Default);
        assert_eq!(q.clone().blue().color, Color::Blue);
        assert_eq!(q.clone().brown().color, Color::Brown);
        assert_eq!(q.clone().gray().color, Color::Gray);
        assert_eq!(q.clone().green().color, Color::Green);
        assert_eq!(q.clone().orange().color, Color::Orange);
        assert_eq!(q.clone().pink().color, Color::Pink);
        assert_eq!(q.clone().purple().color, Color::Purple);
        assert_eq!(q.clone().red().color, Color::Red);
        assert_eq!(q.clone().yellow().color, Color::Yellow);
        assert_eq!(
            q.clone().default_background_color().color,
            Color::DefaultBackground
        );
        assert_eq!(q.clone().blue_background().color, Color::BlueBackground);
        assert_eq!(q.clone().brown_background().color, Color::BrownBackground);
        assert_eq!(q.clone().gray_background().color, Color::GrayBackground);
        assert_eq!(q.clone().green_background().color, Color::GreenBackground);
        assert_eq!(q.clone().orange_background().color, Color::OrangeBackground);
        assert_eq!(q.clone().pink_background().color, Color::PinkBackground);
        assert_eq!(q.clone().purple_background().color, Color::PurpleBackground);
        assert_eq!(q.clone().red_background().color, Color::RedBackground);
        assert_eq!(q.yellow_background().color, Color::YellowBackground);
    }

    #[test]
    fn quote_from_str_and_display() {
        let q: QuoteBlock = "quoted".into();
        assert_eq!(q.to_string(), "quoted");
    }
}
