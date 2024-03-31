use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Deserialize, Serialize, Debug)]
pub struct NotionListResponse<T> {
    // always "list"
    pub object: String,
    pub results: Vec<T>,

    pub next_cursor: Option<String>,
    pub has_more: Option<bool>,
    pub r#type: Option<String>,
    // pub user: Option<String>,
    pub developer_survey: Option<String>,
    pub request_id: Option<String>,
}
