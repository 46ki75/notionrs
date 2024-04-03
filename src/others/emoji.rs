use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/emoji-object
#[derive(Debug, Deserialize, Serialize)]
pub struct Emoji {
    pub r#type: String,
    pub emoji: String,
}
