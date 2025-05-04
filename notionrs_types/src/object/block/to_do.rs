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
}
