use serde::{Deserialize, Serialize};

use crate::color_setters;

/// <https://developers.notion.com/reference/block#headings>
///
/// All heading block objects, heading_1, heading_2, and heading_3,
/// contain the following information within their corresponding objects:
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct HeadingBlock {
    /// The rich text of the heading.
    pub rich_text: Vec<crate::object::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::object::color::Color,

    /// When children are included in the request, it will automatically be set to true.
    /// Whether or not the heading block is a toggle heading or not.
    /// If true, then the heading block toggles and can support children.
    /// If false, then the heading block is a static heading block.
    pub is_toggleable: bool,

    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl HeadingBlock {
    pub fn rich_text(mut self, rich_text: Vec<crate::object::rich_text::RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn children(mut self, children: Vec<super::Block>) -> Self {
        self.children = Some(children);
        self
    }

    color_setters!(self, self.color);

    pub fn is_toggleable(mut self, is_toggleable: bool) -> Self {
        self.is_toggleable = is_toggleable;
        self
    }
}

impl<T> From<T> for HeadingBlock
where
    T: AsRef<str>,
{
    fn from(plain_text: T) -> Self {
        let rich_text = crate::object::rich_text::RichText::from(plain_text.as_ref().to_string());
        Self::default().rich_text(vec![rich_text])
    }
}

impl std::fmt::Display for HeadingBlock {
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

        assert_eq!(heading.color, crate::object::color::Color::Default);

        assert!(!heading.is_toggleable);

        let rich_text = heading.rich_text.first().unwrap();

        match rich_text {
            crate::object::rich_text::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "Heading");
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
    fn heading_color_setters() {
        use crate::object::color::Color;
        let h = HeadingBlock::default();
        assert_eq!(h.clone().default_color().color, Color::Default);
        assert_eq!(h.clone().blue().color, Color::Blue);
        assert_eq!(h.clone().brown().color, Color::Brown);
        assert_eq!(h.clone().gray().color, Color::Gray);
        assert_eq!(h.clone().green().color, Color::Green);
        assert_eq!(h.clone().orange().color, Color::Orange);
        assert_eq!(h.clone().pink().color, Color::Pink);
        assert_eq!(h.clone().purple().color, Color::Purple);
        assert_eq!(h.clone().red().color, Color::Red);
        assert_eq!(h.clone().yellow().color, Color::Yellow);
        assert_eq!(
            h.clone().default_background_color().color,
            Color::DefaultBackground
        );
        assert_eq!(h.clone().blue_background().color, Color::BlueBackground);
        assert_eq!(h.clone().brown_background().color, Color::BrownBackground);
        assert_eq!(h.clone().gray_background().color, Color::GrayBackground);
        assert_eq!(h.clone().green_background().color, Color::GreenBackground);
        assert_eq!(h.clone().orange_background().color, Color::OrangeBackground);
        assert_eq!(h.clone().pink_background().color, Color::PinkBackground);
        assert_eq!(h.clone().purple_background().color, Color::PurpleBackground);
        assert_eq!(h.clone().red_background().color, Color::RedBackground);
        assert_eq!(h.yellow_background().color, Color::YellowBackground);
    }

    #[test]
    fn heading_from_str_and_display() {
        let h: HeadingBlock = "hi".into();
        assert_eq!(h.to_string(), "hi");
    }
}
