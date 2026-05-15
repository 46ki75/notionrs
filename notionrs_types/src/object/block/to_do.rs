use serde::{Deserialize, Serialize};

use crate::color_setters;

/// <https://developers.notion.com/reference/block#to-do>
///
/// To do block objects contain the following information within the to_do property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ToDoBlock {
    /// The rich text displayed in the To do block.
    pub rich_text: Vec<crate::object::rich_text::RichText>,

    /// Whether the To do is checked.
    pub checked: bool,

    /// The color of the block.
    pub color: crate::object::color::Color,
}

impl ToDoBlock {
    color_setters!(self, self.color);
}

impl<T> From<T> for ToDoBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::object::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
    }
}

impl std::fmt::Display for ToDoBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rich_text
                .iter()
                .map(|t| t.to_string())
                .collect::<String>(),
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

        match rich_text {
            crate::object::rich_text::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "TODO");
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
    fn to_do_color_setters() {
        use crate::object::color::Color;
        let t = ToDoBlock::default();
        assert_eq!(t.clone().default_color().color, Color::Default);
        assert_eq!(t.clone().blue().color, Color::Blue);
        assert_eq!(t.clone().brown().color, Color::Brown);
        assert_eq!(t.clone().gray().color, Color::Gray);
        assert_eq!(t.clone().green().color, Color::Green);
        assert_eq!(t.clone().orange().color, Color::Orange);
        assert_eq!(t.clone().pink().color, Color::Pink);
        assert_eq!(t.clone().purple().color, Color::Purple);
        assert_eq!(t.clone().red().color, Color::Red);
        assert_eq!(t.clone().yellow().color, Color::Yellow);
        assert_eq!(
            t.clone().default_background_color().color,
            Color::DefaultBackground
        );
        assert_eq!(t.clone().blue_background().color, Color::BlueBackground);
        assert_eq!(t.clone().brown_background().color, Color::BrownBackground);
        assert_eq!(t.clone().gray_background().color, Color::GrayBackground);
        assert_eq!(t.clone().green_background().color, Color::GreenBackground);
        assert_eq!(t.clone().orange_background().color, Color::OrangeBackground);
        assert_eq!(t.clone().pink_background().color, Color::PinkBackground);
        assert_eq!(t.clone().purple_background().color, Color::PurpleBackground);
        assert_eq!(t.clone().red_background().color, Color::RedBackground);
        assert_eq!(t.yellow_background().color, Color::YellowBackground);
    }

    #[test]
    fn to_do_from_str_and_display() {
        let t: ToDoBlock = "todo".into();
        assert_eq!(t.to_string(), "todo");
    }
}
