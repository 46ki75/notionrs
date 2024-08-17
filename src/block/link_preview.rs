use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#link-preview>
///
/// Link Preview block objects contain the originally pasted url:
#[derive(Deserialize, Serialize, Debug)]
pub struct LinkPreviewBlock {
    pub url: String,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::LinkPreviewBlock;

    #[test]
    fn deserialize_block_child_link_preview() {
        let json_data = r#"
        {
            "url": "example.com"
        }
        "#;

        let link_preview = serde_json::from_str::<LinkPreviewBlock>(json_data).unwrap();

        assert_eq!(link_preview.url, "example.com")
    }
}
