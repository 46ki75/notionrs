use serde::{Deserialize, Serialize};

use crate::{
    object::user::User,
    object::{file::File, icon::Icon, parent::Parent},
};

/// <https://developers.notion.com/reference/page>
#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub properties:
        std::collections::HashMap<String, crate::object::page::properties::PageProperty>,
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
        let json_data = include_str!("./seeds/page_wiki.json");

        let _page = serde_json::from_str::<crate::object::page::PageResponse>(json_data)
            .expect("An error occurred while deserializing the page");
    }

    #[test]
    fn deserialize_page() {
        let json_data = include_str!("./seeds/page.json");

        let _page = serde_json::from_str::<crate::object::page::PageResponse>(json_data)
            .expect("An error occurred while deserializing the page");
    }

    #[test]
    fn deserialize_page_icon_emoji() {
        let json_data = include_str!("./seeds/page_icon_emoji.json");

        let _page = serde_json::from_str::<crate::object::page::PageResponse>(json_data)
            .expect("An error occurred while deserializing the page");
    }
}
