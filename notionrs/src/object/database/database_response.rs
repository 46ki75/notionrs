use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, notionrs_macro::Setter)]
pub struct DatabaseResponse {
    pub id: String,

    pub created_time: String,

    pub last_edited_time: String,

    pub created_by: crate::object::user::User,

    pub last_edited_by: crate::object::user::User,

    pub cover: Option<crate::object::file::File>,

    pub icon: Option<crate::object::icon::Icon>,

    pub url: String,

    pub public_url: Option<String>,

    pub archived: bool,

    pub in_trash: bool,

    pub is_inline: bool,

    pub title: Vec<crate::object::rich_text::RichText>,

    pub description: Vec<crate::object::rich_text::RichText>,

    pub properties: std::collections::HashMap<String, super::properties::DatabaseProperty>,
}
