// https://developers.notion.com/reference/user#bots

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Bot {
    pub object: String, // always "user"
    pub id: String,
    pub name: Option<String>,

    /// Within the Notion API Integration page, when you upload an image,
    /// its storage URL is captured and stored in this field.
    pub avatar_url: Option<String>,

    /// always "bot"
    pub r#type: String,
    pub bot: BotDetail,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BotDetail {
    pub owner: Option<BotOwner>,

    /// "workspace" or "user"
    pub workspace_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BotOwner {
    /// "workspace" or "user"
    pub r#type: String,

    pub workspace: bool,
}
