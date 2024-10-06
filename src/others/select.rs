use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(Debug, Deserialize, Serialize)]
pub struct Select {
    pub id: String,
    pub name: String,
    pub color: Color, // TODO: add restriction (FG)
}

/// <https://developers.notion.com/reference/property-object#status>
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectGroup {
    pub id: String,
    pub name: String,
    pub color: Color, // TODO: add restriction (FG)
    pub option_ids: Vec<String>,
}
