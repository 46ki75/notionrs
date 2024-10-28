use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#template>
///
/// Template blocks represent template buttons in the Notion UI.
/// Template block objects contain the following information within the template property:

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TemplateBlock {
    /// The rich text displayed in the title of the template
    pub rich_text: Vec<crate::others::rich_text::RichText>,
}

impl TemplateBlock {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for TemplateBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rich_text
                .iter()
                .map(|t| t.to_string())
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

    use super::TemplateBlock;

    #[test]
    fn deserialize_block_template() {
        let json_data = r#"
        {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "template item",
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
                    "plain_text": "template item",
                    "href": null
                }
            ]
        }
        "#;

        let template: TemplateBlock = serde_json::from_str::<TemplateBlock>(json_data).unwrap();

        let rich_text = template.rich_text.first().unwrap();

        match rich_text {
            crate::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "template item");
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
