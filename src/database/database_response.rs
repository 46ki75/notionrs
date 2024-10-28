use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseResponse {
    pub id: String,

    pub created_time: String,

    pub last_edited_time: String,

    pub created_by: crate::user::User,

    pub last_edited_by: crate::user::User,

    pub cover: Option<crate::others::file::File>,

    pub icon: Option<crate::others::icon::Icon>,

    pub url: String,

    pub public_url: Option<String>,

    pub archived: bool,

    pub in_trash: bool,

    pub is_inline: bool,

    pub title: Vec<crate::others::rich_text::RichText>,

    pub description: Vec<crate::others::rich_text::RichText>,

    pub properties:
        std::collections::HashMap<String, crate::database::properties::DatabaseProperty>,
}
