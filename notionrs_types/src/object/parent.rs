use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/parent-object>
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Parent {
    DatabaseParent(DatabaseParent),
    DataSourceParent(DataSourceParent),
    PageParent(PageParent),
    WorkspaceParent(WorkspaceParent),
    BlockParent(BlockParent),
    AgentIdParent(AgentIdParent),
}

/// <https://developers.notion.com/reference/parent-object#database-parent>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DatabaseParent {
    /// always "database_id"
    #[serde(skip_serializing)]
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

/// <https://developers.notion.com/reference/parent-object#data-source-parent>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataSourceParent {
    /// always "data_source_id"
    #[serde(skip_serializing)]
    pub r#type: String,
    pub data_source_id: String,
}

impl From<&str> for DataSourceParent {
    fn from(data_source_id: &str) -> Self {
        Self {
            r#type: "data_source_id".to_string(),
            data_source_id: data_source_id.to_string(),
        }
    }
}

impl From<String> for DataSourceParent {
    fn from(data_source_id: String) -> Self {
        Self {
            r#type: "data_source_id".to_string(),
            data_source_id: data_source_id,
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
    #[serde(skip_serializing)]
    pub r#type: String,
    /// always `true`
    pub workspace: bool,
}

/// <https://developers.notion.com/reference/parent-object#block-parent>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BlockParent {
    /// always "block_id"
    #[serde(skip_serializing)]
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

/// Parent type for workflow-parented pages and blocks.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AgentIdParent {
    /// always "agent_id"
    #[serde(skip_serializing)]
    pub r#type: String,
    pub agent_id: String,
}

// # --------------------------------------------------------------------------------
//
// Unit tests
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_agent_id_parent() {
        let json = r#"{"type":"agent_id","agent_id":"abc123"}"#;
        let parent: Parent = serde_json::from_str(json).unwrap();
        assert!(matches!(parent, Parent::AgentIdParent(_)));
        if let Parent::AgentIdParent(p) = parent {
            assert_eq!(p.agent_id, "abc123");
        }
    }

    #[test]
    fn deserialize_block_parent() {
        let json = r#"{"type":"block_id","block_id":"block-abc"}"#;
        let parent: Parent = serde_json::from_str(json).unwrap();
        assert!(matches!(parent, Parent::BlockParent(_)));
    }

    #[test]
    fn parent_variants_from_str_and_string() {
        let dbp: DatabaseParent = "db1".into();
        assert_eq!(dbp.r#type, "database_id");
        assert_eq!(dbp.database_id, "db1");
        let dbp2: DatabaseParent = "db2".to_string().into();
        assert_eq!(dbp2.database_id, "db2");

        let dsp: DataSourceParent = "ds1".into();
        assert_eq!(dsp.r#type, "data_source_id");
        let dsp2: DataSourceParent = "ds2".to_string().into();
        assert_eq!(dsp2.data_source_id, "ds2");

        let pp: PageParent = "p1".into();
        assert_eq!(pp.r#type, "page_id");
        let pp2: PageParent = "p2".to_string().into();
        assert_eq!(pp2.page_id, "p2");

        let bp: BlockParent = "b1".into();
        assert_eq!(bp.r#type, "block_id");
        let bp2: BlockParent = "b2".to_string().into();
        assert_eq!(bp2.block_id, "b2");

        let _ = WorkspaceParent::default();
        let _ = AgentIdParent::default();
    }

    #[test]
    fn parent_enum_deserialization() {
        let dbp =
            serde_json::from_str::<Parent>(r#"{"type":"database_id","database_id":"x"}"#).unwrap();
        assert!(matches!(dbp, Parent::DatabaseParent(_)));

        let dsp =
            serde_json::from_str::<Parent>(r#"{"type":"data_source_id","data_source_id":"x"}"#)
                .unwrap();
        assert!(matches!(dsp, Parent::DataSourceParent(_)));

        let pp = serde_json::from_str::<Parent>(r#"{"type":"page_id","page_id":"x"}"#).unwrap();
        assert!(matches!(pp, Parent::PageParent(_)));

        let wp =
            serde_json::from_str::<Parent>(r#"{"type":"workspace","workspace":true}"#).unwrap();
        assert!(matches!(wp, Parent::WorkspaceParent(_)));
    }
}
