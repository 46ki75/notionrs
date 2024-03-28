use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NotionResponse<T> {
    pub object: String,
    pub results: Vec<T>,

    pub next_cursor: Option<String>,
    pub has_more: Option<bool>,
    pub r#type: Option<String>,
    // pub user: Option<String>,
    pub developer_survey: Option<String>,
    pub request_id: Option<String>,
}
