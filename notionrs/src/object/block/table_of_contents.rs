use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#table-of-contents>
///
/// Table of contents block objects contain the following information within the table_of_contents property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy, notionrs_macro::Setter)]
pub struct TableOfContentsBlock {
    /// The color of the block. Possible values are:
    pub color: crate::object::color::Color,
}

impl std::fmt::Display for TableOfContentsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.color)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::TableOfContentsBlock;

    #[test]
    fn deserialize_block_table() {
        let json_data = r#"
        {
            "color": "red"
        }
        "#;

        let table = serde_json::from_str::<TableOfContentsBlock>(json_data).unwrap();

        assert_eq!(table.color, crate::object::color::Color::Red);
    }
}
