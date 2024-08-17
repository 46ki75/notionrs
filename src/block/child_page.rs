use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#child-page>
///
/// Child database block objects contain the following
/// information within the child_page property:
#[derive(Deserialize, Serialize, Debug)]
pub struct ChildPageBlock {
    /// The plain text title of the page.
    pub title: String,
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
