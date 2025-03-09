use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/parent-object>
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Parent {
    DatabaseParent(DatabaseParent),
    PageParent(PageParent),
    WorkspaceParent(WorkspaceParent),
    BlockParent(BlockParent),
}

/// <https://developers.notion.com/reference/parent-object#database-parent>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DatabaseParent {
    /// always "database_id"
    pub r#type: String,
    pub database_id: String,
}

impl From<&str> for DatabaseParent {
    fn from(database_id: &str) -> Self {
        Self {
            r#type: "database_id".to_string(),
            database_id: database_id.to_string(),
        }
    }
}

impl From<String> for DatabaseParent {
    fn from(database_id: String) -> Self {
        Self {
            r#type: "database_id".to_string(),
            database_id,
        }
    }
}

/// <https://developers.notion.com/reference/parent-object#page-parent>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PageParent {
    /// always "page_id"
    pub r#type: String,
    pub page_id: String,
}

impl From<&str> for PageParent {
    fn from(page_id: &str) -> Self {
        Self {
            r#type: "page_id".to_string(),
            page_id: page_id.to_string(),
        }
    }
}

impl From<String> for PageParent {
    fn from(page_id: String) -> Self {
        Self {
            r#type: "page_id".to_string(),
            page_id,
        }
    }
}

/// <https://developers.notion.com/reference/parent-object#workspace-parent>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WorkspaceParent {
    /// always "workspace"
    pub r#type: String,
    /// always `true`
    pub workspace: bool,
}

/// <https://developers.notion.com/reference/parent-object#block-parent>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BlockParent {
    /// always "block_id"
    pub r#type: String,
    pub block_id: String,
}

impl From<&str> for BlockParent {
    fn from(block_id: &str) -> Self {
        Self {
            r#type: "block_id".to_string(),
            block_id: block_id.to_string(),
        }
    }
}

impl From<String> for BlockParent {
    fn from(block_id: String) -> Self {
        Self {
            r#type: "block_id".to_string(),
            block_id,
        }
    }
}
