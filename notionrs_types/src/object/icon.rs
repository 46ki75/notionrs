use core::fmt;

use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/emoji-object#icon>
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Icon {
    /// The constant string "icon" that represents the object type.
    pub r#type: String,

    /// An object with name (required) and color (optional, defaults to "gray"). See below for valid values.
    pub icon: IconContent,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct IconContent {
    /// The name field identifies the icon. Refer to the Notion icon picker for valid names (e.g. "pizza", "meeting", "home").
    pub name: String,

    pub color: IconColor,
}

/// <https://developers.notion.com/reference/emoji-object#icon-color>
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum IconColor {
    Blue,
    Brown,
    #[default]
    Gray,
    Green,
    #[serde(rename = "light_gray")]
    Lightgray,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,
}

impl fmt::Display for IconContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icon)
    }
}
