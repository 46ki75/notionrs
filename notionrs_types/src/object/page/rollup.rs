use serde::{Deserialize, Serialize};

// TODO: Implement the Rollup object.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageRollupProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl std::fmt::Display for PageRollupProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    use super::*;

    #[test]
    fn page_rollup_property() {
        let json = r#"{"id":"ro1"}"#;
        let prop: PageRollupProperty = serde_json::from_str(json).unwrap();
        assert_eq!(prop.id.as_deref(), Some("ro1"));
        assert_eq!(prop.to_string(), "");
    }
}
