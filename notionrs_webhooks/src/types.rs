#![allow(deprecated)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub timestamp: String,
    pub workspace_id: String,
    pub workspace_name: String,
    pub subscription_id: String,
    pub integration_id: String,
    pub authors: Vec<WebhookAuthor>,
    pub accessible_by: Option<Vec<WebhookAuthor>>,
    pub attempt_number: u32,
    pub entity: WebhookEntity,
    #[serde(flatten)]
    pub data: EventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum WebhookAuthor {
    Person { id: String },
    Bot { id: String },
    Agent { id: String },
}

impl WebhookAuthor {
    pub fn get_id(self) -> String {
        match self {
            WebhookAuthor::Person { id } => id,
            WebhookAuthor::Bot { id } => id,
            WebhookAuthor::Agent { id } => id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum WebhookEntity {
    Page { id: String },
    Block { id: String },
    Database { id: String },
    DataSource { id: String },
    Space { id: String },
    Comment { id: String },
}

impl WebhookEntity {
    pub fn get_id(self) -> String {
        match self {
            WebhookEntity::Page { id } => id,
            WebhookEntity::Block { id } => id,
            WebhookEntity::Database { id } => id,
            WebhookEntity::DataSource { id } => id,
            WebhookEntity::Space { id } => id,
            WebhookEntity::Comment { id } => id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum WebhookProperties {
    Created { id: String, name: String },
    Updated { id: String, name: String },
    Deleted { id: String, name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventData {
    #[serde(rename = "page.created")]
    PageCreated(PageCreated),

    #[serde(rename = "page.properties_updated")]
    PagePropertiesUpdated(PagePropertiesUpdated),

    #[serde(rename = "page.content_updated")]
    PageContentUpdated(PageContentUpdated),

    #[serde(rename = "page.moved")]
    PageMoved(PageMoved),

    #[serde(rename = "page.deleted")]
    PageDeleted(PageDeleted),

    #[serde(rename = "page.undeleted")]
    PageUndeleted(PageUndeleted),

    #[serde(rename = "page.locked")]
    PageLocked(PageLocked),

    #[serde(rename = "page.unlocked")]
    PageUnlocked(PageUnlocked),

    #[serde(rename = "database.created")]
    DatabaseCreated(DatabaseCreated),

    #[serde(rename = "database.content_updated")]
    DatabaseContentUpdated(DatabaseContentUpdated),

    #[serde(rename = "database.moved")]
    DatabaseMoved(DatabaseMoved),

    #[serde(rename = "database.deleted")]
    DatabaseDeleted(DatabaseDeleted),

    #[serde(rename = "database.undeleted")]
    DatabaseUndeleted(DatabaseUndeleted),

    #[serde(rename = "database.schema_updated")]
    DatabaseSchemaUpdated(DatabaseSchemaUpdated),

    #[serde(rename = "data_source.content_updated")]
    DataSourceContentUpdated(DataSourceContentUpdated),

    #[serde(rename = "data_source.created")]
    DataSourceCreated(DataSourceCreated),

    #[serde(rename = "data_source.deleted")]
    DataSourceDeleted(DataSourceDeleted),

    #[serde(rename = "data_source.moved")]
    DataSourceMoved(DataSourceMoved),

    #[serde(rename = "data_source.schema_updated")]
    DataSourceSchemaUpdated(DataSourceSchemaUpdated),

    #[serde(rename = "data_source.undeleted")]
    DataSourceUndeleted(DataSourceUndeleted),

    #[serde(rename = "comment.created")]
    CommentCreated(CommentCreated),

    #[serde(rename = "comment.updated")]
    CommentUpdated(CommentUpdated),

    #[serde(rename = "comment.deleted")]
    CommentDeleted(CommentDeleted),
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pagecreated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageCreated {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pageproperties_updated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagePropertiesUpdated {
    pub parent: WebhookEntity,
    pub updated_properties: Vec<String>,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pagecontent_updated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageContentUpdated {
    pub parent: WebhookEntity,
    pub updated_blocks: Vec<WebhookEntity>,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pagemoved>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMoved {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pagedeleted>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDeleted {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pageundeleted>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageUndeleted {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pagelocked>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLocked {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#pageunlocked>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageUnlocked {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#databasecreated>
/// For linked databases, the `entity.type` is `"block"` instead of `"database"`.
/// If you retrieve this block in the API, it has a type of `"child_database"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCreated {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#databasecontent_updated>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "Deprecated in 2025-09-03 API version.")]
pub struct DatabaseContentUpdated {
    pub parent: WebhookEntity,
    pub updated_blocks: Vec<WebhookEntity>,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#databasemoved>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMoved {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#databasedeleted>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseDeleted {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#databaseundeleted>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseUndeleted {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#databaseschema_updated>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "Deprecated in 2025-09-03 API version.")]
pub struct DatabaseSchemaUpdated {
    pub parent: WebhookEntity,
    pub updated_properties: Vec<WebhookProperties>,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#data_sourcecontent_updated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceContentUpdated {
    pub parent: WebhookEntity,
    pub updated_blocks: Vec<WebhookEntity>,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#data_sourcecreated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceCreated {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#data_sourcedeleted>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceDeleted {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#data_sourcemoved>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceMoved {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#data_sourceschema_updated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceSchemaUpdated {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#data_sourceundeleted>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceUndeleted {
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#commentcreated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCreated {
    pub page_id: String,
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#commentupdated>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentUpdated {
    pub page_id: String,
    pub parent: WebhookEntity,
}

/// <https://developers.notion.com/reference/webhooks-events-delivery#commentdeleted>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDeleted {
    pub page_id: String,
    pub parent: WebhookEntity,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_page_created() {
        let json_data = include_bytes!("./events/page.created.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::PageCreated(_)));
    }

    #[test]
    fn deserialize_page_properties_updated() {
        let json_data = include_bytes!("./events/page.properties_updated.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(
            deserialized.data,
            EventData::PagePropertiesUpdated(_)
        ));
    }

    #[test]
    fn deserialize_page_content_updated() {
        let json_data = include_bytes!("./events/page.content_updated.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(
            deserialized.data,
            EventData::PageContentUpdated(_)
        ));
    }

    #[test]
    fn deserialize_page_moved() {
        let json_data = include_bytes!("./events/page.moved.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::PageMoved(_)));
    }

    #[test]
    fn deserialize_page_deleted() {
        let json_data = include_bytes!("./events/page.deleted.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::PageDeleted(_)));
    }

    #[test]
    fn deserialize_page_undeleted() {
        let json_data = include_bytes!("./events/page.undeleted.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::PageUndeleted(_)));
    }

    #[test]
    fn deserialize_page_locked() {
        let json_data = include_bytes!("./events/page.locked.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::PageLocked(_)));
    }

    #[test]
    fn deserialize_page_unlocked() {
        let json_data = include_bytes!("./events/page.unlocked.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::PageUnlocked(_)));
    }

    #[test]
    fn deserialize_database_created() {
        let json_data = include_bytes!("./events/database.created.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::DatabaseCreated(_)));
    }

    #[test]
    fn deserialize_database_content_updated() {
        let json_data = include_bytes!("./events/database.content_updated.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(
            deserialized.data,
            EventData::DatabaseContentUpdated(_)
        ));
    }

    #[test]
    fn deserialize_database_moved() {
        let json_data = include_bytes!("./events/database.moved.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::DatabaseMoved(_)));
    }

    #[test]
    fn deserialize_database_deleted() {
        let json_data = include_bytes!("./events/database.deleted.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::DatabaseDeleted(_)));
    }

    #[test]
    fn deserialize_database_undeleted() {
        let json_data = include_bytes!("./events/database.undeleted.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::DatabaseUndeleted(_)));
    }

    #[test]
    fn deserialize_database_schema_updated() {
        let json_data = include_bytes!("./events/database.schema_updated.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(
            deserialized.data,
            EventData::DatabaseSchemaUpdated(_)
        ));
    }

    #[test]
    fn deserialize_data_source_content_updated() {
        let json_data = include_bytes!("./events/data_source.content_updated.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(
            deserialized.data,
            EventData::DataSourceContentUpdated(_)
        ));
    }

    #[test]
    fn deserialize_data_source_created() {
        let json_data = include_bytes!("./events/data_source.created.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::DataSourceCreated(_)));
    }

    #[test]
    fn deserialize_data_source_deleted() {
        let json_data = include_bytes!("./events/data_source.deleted.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::DataSourceDeleted(_)));
    }

    #[test]
    fn deserialize_data_source_moved() {
        let json_data = include_bytes!("./events/data_source.moved.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::DataSourceMoved(_)));
    }

    #[test]
    fn deserialize_data_source_schema_updated() {
        let json_data = include_bytes!("./events/data_source.schema_updated.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(
            deserialized.data,
            EventData::DataSourceSchemaUpdated(_)
        ));
    }

    #[test]
    fn deserialize_data_source_undeleted() {
        let json_data = include_bytes!("./events/data_source.undeleted.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(
            deserialized.data,
            EventData::DataSourceUndeleted(_)
        ));
    }

    #[test]
    fn deserialize_comment_created() {
        let json_data = include_bytes!("./events/comment.created.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::CommentCreated(_)));
    }

    #[test]
    fn deserialize_comment_updated() {
        let json_data = include_bytes!("./events/comment.updated.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::CommentUpdated(_)));
    }

    #[test]
    fn deserialize_comment_deleted() {
        let json_data = include_bytes!("./events/comment.deleted.json");
        let deserialized = serde_json::from_slice::<WebhookEvent>(json_data).unwrap();
        println!("{:#?}", deserialized);
        assert!(matches!(deserialized.data, EventData::CommentDeleted(_)));
    }
}
