use serde::{Deserialize, Serialize};

use crate::others::rich_text::RichText;

#[derive(Debug, Deserialize, Serialize)]
pub struct PageTitle {
    pub id: String,
    pub r#type: String,
    pub title: Vec<RichText>,
}
