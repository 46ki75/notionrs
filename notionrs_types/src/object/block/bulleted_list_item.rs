use serde::{Deserialize, Serialize};

use crate::color_setters;

/// <https://developers.notion.com/reference/block#bulleted-list-item>
///
///  Bulleted list item block objects contain the following
/// information within the bulleted_list_item property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct BulletedListItemBlock {
    /// The rich text in the bulleted_list_item block.
    pub rich_text: Vec<crate::object::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::object::color::Color,

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl BulletedListItemBlock {
    color_setters!(self, self.color);
}

impl From<Vec<crate::object::rich_text::RichText>> for BulletedListItemBlock {
    fn from(rich_text: Vec<crate::object::rich_text::RichText>) -> Self {
        Self::default().rich_text(rich_text)
    }
}

impl std::fmt::Display for BulletedListItemBlock {
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
            crate::object::color::Color::Default
        );

        let rich_text = bulleted_list_item.rich_text.first().unwrap();

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
    fn bulleted_color_setters() {
        use crate::object::color::Color;
        let b = BulletedListItemBlock::default();
        assert_eq!(b.clone().default_color().color, Color::Default);
        assert_eq!(b.clone().blue().color, Color::Blue);
        assert_eq!(b.clone().brown().color, Color::Brown);
        assert_eq!(b.clone().gray().color, Color::Gray);
        assert_eq!(b.clone().green().color, Color::Green);
        assert_eq!(b.clone().orange().color, Color::Orange);
        assert_eq!(b.clone().pink().color, Color::Pink);
        assert_eq!(b.clone().purple().color, Color::Purple);
        assert_eq!(b.clone().red().color, Color::Red);
        assert_eq!(b.clone().yellow().color, Color::Yellow);
        assert_eq!(
            b.clone().default_background_color().color,
            Color::DefaultBackground
        );
        assert_eq!(b.clone().blue_background().color, Color::BlueBackground);
        assert_eq!(b.clone().brown_background().color, Color::BrownBackground);
        assert_eq!(b.clone().gray_background().color, Color::GrayBackground);
        assert_eq!(b.clone().green_background().color, Color::GreenBackground);
        assert_eq!(b.clone().orange_background().color, Color::OrangeBackground);
        assert_eq!(b.clone().pink_background().color, Color::PinkBackground);
        assert_eq!(b.clone().purple_background().color, Color::PurpleBackground);
        assert_eq!(b.clone().red_background().color, Color::RedBackground);
        assert_eq!(b.yellow_background().color, Color::YellowBackground);
    }

    #[test]
    fn bulleted_from_vec_rich_text_and_display() {
        use crate::object::rich_text::RichText;
        let b: BulletedListItemBlock = vec![RichText::from("hi")].into();
        assert_eq!(b.to_string(), "hi");
    }
}
