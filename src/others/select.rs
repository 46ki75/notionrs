use serde::{Deserialize, Serialize};

use super::color::ColorFG;

#[derive(Debug, Deserialize, Serialize)]
pub struct Select {
    pub id: String,
    pub name: String,
    pub color: ColorFG,
}

/// <https://developers.notion.com/reference/property-object#status>
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectGroup {
    pub id: String,
    pub name: String,
    pub color: ColorFG,
    pub option_ids: Vec<String>,
}
