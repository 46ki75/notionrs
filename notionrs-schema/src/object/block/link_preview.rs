use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#link-preview>
///
/// Link Preview block objects contain the originally pasted url:
///
/// The link_preview block can only be returned as part of a response.
/// The API does not support creating or appending link_preview blocks.
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct LinkPreviewBlock {
    pub url: String,
}

impl std::fmt::Display for LinkPreviewBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
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
