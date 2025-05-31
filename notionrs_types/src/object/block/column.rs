use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, notionrs_macro::Setter)]
pub struct ColumnBlock {
    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,

    pub width_ratio: f64,
}

impl Default for ColumnBlock {
    fn default() -> Self {
        Self {
            children: None,
            width_ratio: 0.5,
        }
    }
}

impl std::fmt::Display for ColumnBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_block_column() {
        let json_data = r#"{"width_ratio": 0.5}"#;

        let column = serde_json::from_str::<ColumnBlock>(json_data).unwrap();

        assert!(column.children.is_none());
        assert_eq!(column.width_ratio, 0.5);
    }
}
