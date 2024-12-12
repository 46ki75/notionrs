use serde::{Deserialize, Serialize};

use crate::{
    others::{file::File, icon::Icon, parent::Parent},
    user::User,
};

/// <https://developers.notion.com/reference/page>
#[derive(Deserialize, Serialize, Debug)]
pub struct PageResponse {
    pub id: String,
    pub created_time: chrono::DateTime<chrono::FixedOffset>,
    pub last_edited_time: chrono::DateTime<chrono::FixedOffset>,
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

// # --------------------------------------------------------------------------------
//
// unit_tests
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    #[test]
    fn deserialize_wiki_page() {
        let json_data = include_str!("./seeds/wiki_page.json");

        let _page = serde_json::from_str::<crate::page::PageResponse>(json_data)
            .expect("An error occurred while deserializing the page");
    }
}
