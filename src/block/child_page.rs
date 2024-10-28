use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#child-page>
///
/// To create or update child_page type blocks,
/// use the [Create a page](https://developers.notion.com/reference/post-page)
/// and the [Update page](https://developers.notion.com/reference/patch-page) endpoints,
/// specifying the ID of the parent page in the parent body param.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChildPageBlock {
    /// The plain text title of the page.
    pub title: String,
}

impl std::fmt::Display for ChildPageBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::ChildPageBlock;

    #[test]
    fn deserialize_block_child_page() {
        let json_data = r#"
        {
            "title": "My Page"
        }
        "#;

        let child_page = serde_json::from_str::<ChildPageBlock>(json_data).unwrap();

        assert_eq!(child_page.title, "My Page")
    }
}
