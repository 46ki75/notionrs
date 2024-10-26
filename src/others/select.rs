use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Select {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<SelectColor>,
}

impl Select {
    pub fn id<T>(mut self, id: T) -> Self
    where
        T: AsRef<str>,
    {
        self.id = Some(id.as_ref().to_string());
        self
    }

    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        self.name = name.as_ref().to_string();
        self
    }

    pub fn color(mut self, color: SelectColor) -> Self {
        self.color = Some(color);
        self
    }
}

impl<T> From<T> for Select
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self::default().name(value)
    }
}

/// <https://developers.notion.com/reference/property-object#status>
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
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
