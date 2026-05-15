use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#tab>
///
/// Tab block objects represent a tabbed container. The `tab` property is an empty object (`{}`).
/// Each tab within the container is a [paragraph](https://developers.notion.com/reference/block#paragraph)
/// child block — the paragraph’s `rich_text` serves as the tab label,
/// and the paragraph’s `children` contain the tab’s content.
///
/// Only `paragraph` blocks can be direct children of a tab block. When creating tabs via [Append block children](https://developers.notion.com/reference/patch-block-children), provide a `tab` block with paragraph children, each containing their own nested content blocks.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TabBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl std::fmt::Display for TabBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::TabBlock;

    #[test]
    fn deserialize_block_child_page() {
        let json_data = r#"
        {}
        "#;

        let _ = serde_json::from_str::<TabBlock>(json_data).unwrap();
    }

    #[test]
    fn tab_display() {
        let tab = TabBlock {
            children: Some(vec![]),
        };
        assert_eq!(tab.to_string(), "");
    }
}
