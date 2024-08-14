use serde::{Deserialize, Serialize};

// TODO: Implement the Rollup object.
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRollupProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // TODO: test
}
