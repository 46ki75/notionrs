use serde::{Deserialize, Serialize};

use crate::others::rich_text::RichText;

/// https://developers.notion.com/reference/page-property-values#title
#[derive(Debug, Deserialize, Serialize)]
pub struct PageTitle {
    pub id: String,
    pub title: Vec<RichText>,
}
