use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListResponse<T> {
    /// always "list"
    pub object: String,
    pub results: Vec<T>,

    pub next_cursor: Option<String>,
    pub has_more: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SearchResultItem {
    Page(crate::page::PageResponse),
    Database(crate::database::DatabaseResponse),
}
