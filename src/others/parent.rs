use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/parent-object
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Parent {
    DatabaseParent(DatabaseParent),
    PageParent(PageParent),
    WorkspaceParent(WorkspaceParent),
    BlockParent(BlockParent),
}

/// https://developers.notion.com/reference/parent-object#database-parent
#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseParent {
    /// always "database_id"
    pub r#type: String,
    pub database_id: String,
}

/// https://developers.notion.com/reference/parent-object#page-parent
#[derive(Serialize, Deserialize, Debug)]
pub struct PageParent {
    /// always "page_id"
    pub r#type: String,
    pub page_id: String,
}

/// https://developers.notion.com/reference/parent-object#workspace-parent
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceParent {
    /// always "workspace"
    pub r#type: String,
    /// always `true`
    pub workspace: bool,
}

/// https://developers.notion.com/reference/parent-object#block-parent
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockParent {
    /// always "block_id"
    pub r#type: String,
    pub block_id: String,
}
