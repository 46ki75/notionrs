use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#template>
///
/// Template blocks represent template buttons in the Notion UI.
/// Template block objects contain the following information within the template property:

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TemplateBlock {
    /// The rich text displayed in the title of the template
    pub rich_text: Vec<crate::others::rich_text::RichText>,
}

impl TemplateBlock {
    pub fn build(self) -> super::Block {
        super::Block::Template { template: self }
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

        assert_eq!(rich_text.plain_text, "template item");
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
