use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#numbered-list-item>
///
/// Numbered list item block objects contain the following
/// information within the numbered_list_item property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct NumberedListItemBlock {
    /// The rich text displayed in the numbered_list_item block.
    pub rich_text: Vec<crate::object::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::object::color::Color,

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,

    /// The type of list format. Possible values are: `"numbers"`, `"letters"`, and `"roman"`.
    /// Only present on the first item of a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_format: Option<NumberedListFormat>,

    /// The start index of a list, used to represent a list that doesn’t start at 1.
    /// Only present on the first item of a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_start_index: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NumberedListFormat {
    Numbers,
    Letters,
    Roman,
}

impl NumberedListItemBlock {
    crate::color_setters!(self, self.color);
}

impl<T> From<T> for NumberedListItemBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::object::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
    }
}

impl std::fmt::Display for NumberedListItemBlock {
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
            "color": "default",
            "list_format": "roman",
            "list_start_index": 2
        }
        "#;

        let numbered_list_item: NumberedListItemBlock =
            serde_json::from_str::<NumberedListItemBlock>(json_data).unwrap();

        assert_eq!(
            numbered_list_item.color,
            crate::object::color::Color::Default
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
        //     crate::object::color::Color::Default
        // );

        match rich_text {
            crate::object::rich_text::RichText::Text {
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
                assert_eq!(annotations.color, crate::object::color::Color::Default)
            }
            _ => panic!(),
        }
    }

    #[test]
    fn numbered_color_setters() {
        use crate::object::color::Color;
        let n = NumberedListItemBlock::default();
        assert_eq!(n.clone().default_color().color, Color::Default);
        assert_eq!(n.clone().blue().color, Color::Blue);
        assert_eq!(n.clone().brown().color, Color::Brown);
        assert_eq!(n.clone().gray().color, Color::Gray);
        assert_eq!(n.clone().green().color, Color::Green);
        assert_eq!(n.clone().orange().color, Color::Orange);
        assert_eq!(n.clone().pink().color, Color::Pink);
        assert_eq!(n.clone().purple().color, Color::Purple);
        assert_eq!(n.clone().red().color, Color::Red);
        assert_eq!(n.clone().yellow().color, Color::Yellow);
        assert_eq!(
            n.clone().default_background_color().color,
            Color::DefaultBackground
        );
        assert_eq!(n.clone().blue_background().color, Color::BlueBackground);
        assert_eq!(n.clone().brown_background().color, Color::BrownBackground);
        assert_eq!(n.clone().gray_background().color, Color::GrayBackground);
        assert_eq!(n.clone().green_background().color, Color::GreenBackground);
        assert_eq!(n.clone().orange_background().color, Color::OrangeBackground);
        assert_eq!(n.clone().pink_background().color, Color::PinkBackground);
        assert_eq!(n.clone().purple_background().color, Color::PurpleBackground);
        assert_eq!(n.clone().red_background().color, Color::RedBackground);
        assert_eq!(n.yellow_background().color, Color::YellowBackground);
    }

    #[test]
    fn numbered_from_str_and_display() {
        let n: NumberedListItemBlock = "num".into();
        assert_eq!(n.to_string(), "num");
    }
}
