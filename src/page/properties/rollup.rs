use serde::{Deserialize, Serialize};

// TODO: Implement the Rollup object.
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRollupProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    // TODO: test
}
