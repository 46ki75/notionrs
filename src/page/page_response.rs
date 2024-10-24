use serde::{Deserialize, Serialize};

use crate::{
    others::{file::File, icon::Icon, parent::Parent},
    user::User,
};

/// <https://developers.notion.com/reference/page>
#[derive(Deserialize, Serialize, Debug)]
pub struct PageResponse {
    pub id: String,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub last_edited_time: chrono::DateTime<chrono::Utc>,
    pub created_by: User,
    pub last_edited_by: User,
    pub cover: Option<File>,
    pub icon: Option<Icon>,
    pub parent: Parent,
    pub archived: bool,
    pub properties: std::collections::HashMap<String, crate::page::properties::PageProperty>,
    pub url: String,
    pub public_url: Option<String>,
    pub developer_survey: Option<String>,
    pub request_id: Option<String>,
    pub in_trash: bool,
}
