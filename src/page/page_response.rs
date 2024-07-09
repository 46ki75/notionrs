use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    others::{file::File, icon::Icon, parent::Parent},
    user::User,
};

use super::properties::PageProperty;

/// https://developers.notion.com/reference/page
#[derive(Deserialize, Serialize, Debug)]
pub struct PageResponse {
    pub id: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub created_by: User,
    pub last_edited_by: User,
    pub cover: Option<File>,
    pub icon: Option<Icon>,
    pub parent: Parent,
    pub archived: bool,
    pub properties: HashMap<String, PageProperty>,
    pub url: String,
    pub public_url: Option<String>,
    pub developer_survey: Option<String>,
    pub request_id: Option<String>,
    pub in_trash: bool,
}
