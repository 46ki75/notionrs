use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Select {
    pub id: String,
    pub name: String,
    pub color: SelectColor,
}

/// <https://developers.notion.com/reference/property-object#status>
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectGroup {
    pub id: String,
    pub name: String,
    pub color: SelectColor,
    pub option_ids: Vec<String>,
}

/// Specifies the label colors. These colors can be used in the following property types.
/// This color enumeration is different from the usual Color (which includes Background Color).
///
/// - [Status (docs)](https://developers.notion.com/reference/property-object#status)
/// - [Select (docs)](https://developers.notion.com/reference/property-object#select)
/// - [Multi-select (docs)](https://developers.notion.com/reference/property-object#multi-select)
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SelectColor {
    #[default]
    Default,

    Blue,
    Brown,
    Gray,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,
}
