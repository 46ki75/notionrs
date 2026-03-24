use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#unsupported>
///
/// When a block type is not yet supported by the API, the response includes `type`
/// set to `"unsupported"` and an `unsupported` object with a `block_type` field.
/// The `block_type` value is a plain string identifying the underlying internal block type
/// (e.g., `"form"`, `"button"`, `"tab"`). This field is informational only
/// and does not expose block content.
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct UnsupportedBlock {
    pub block_type: String,
}

impl std::fmt::Display for UnsupportedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.block_type)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::UnsupportedBlock;

    #[test]
    fn deserialize_block_unsupported() {
        let json_data = r#"
        {
            "block_type": "button"
        }
        "#;

        let link_preview = serde_json::from_str::<UnsupportedBlock>(json_data).unwrap();

        assert_eq!(link_preview.block_type, "button")
    }
}
