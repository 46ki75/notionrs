use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#synced-block>
///
/// Similar to the Notion UI, there are two versions of a synced_block object:
/// the original block that was created first and doesn't yet sync with anything else,
/// and the duplicate block or blocks synced to the original.
///
/// First, set `synced_from` to null and add the blocks you want to sync to the children,
/// then send the request. After that, you can create two or more synced blocks
/// by sending a request with the ID of the initially created block set in `synced_from.block_id`.
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct SyncedBlock {
    pub synced_from: Option<SyncedBlockParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SyncedBlockParams {
    /// always "block_id"
    pub r#type: String,

    /// An identifier for the original synced_block.
    pub block_id: String,
}

impl SyncedBlock {
    /// An identifier for the original synced_block.
    pub fn block_id<T>(mut self, block_id: T) -> Self
    where
        T: AsRef<str>,
    {
        self.synced_from = Some(SyncedBlockParams {
            r#type: "block_id".to_string(),
            block_id: block_id.as_ref().to_string(),
        });
        self
    }
}

impl Default for SyncedBlockParams {
    fn default() -> Self {
        Self {
            r#type: "block_id".to_string(),
            block_id: String::default(),
        }
    }
}

impl<T> From<T> for SyncedBlock
where
    T: AsRef<str>,
{
    fn from(block_id: T) -> Self {
        Self::default().block_id(block_id)
    }
}

impl std::fmt::Display for SyncedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.synced_from {
            Some(synced_from) => write!(f, "{}", synced_from.block_id),
            None => write!(f, ""),
        }
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::SyncedBlock;

    #[test]
    fn deserialize_block_synced() {
        let json_data = r#"
        {
            "synced_from": {
                "type": "block_id",
                "block_id": "9c71962d-8c9a-4bdf-b1a1-2f5cec3ac454"
            }
        }
        "#;

        let synced_block = serde_json::from_str::<SyncedBlock>(json_data).unwrap();

        assert_eq!(
            synced_block.synced_from.unwrap().block_id,
            "9c71962d-8c9a-4bdf-b1a1-2f5cec3ac454"
        )
    }
}
