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

impl std::fmt::Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
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

impl std::fmt::Display for SelectGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
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

impl std::str::FromStr for SelectColor {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str(s)
    }
}

impl std::fmt::Display for SelectColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_plain::to_string(self)
            .map_err(|_| std::fmt::Error)?
            .fmt(f)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn deserialize_select() {
        let json = r#"{
            "id": "id",
            "name": "name",
            "color": "blue"
        }"#;
        let select: Select = serde_json::from_str(json).unwrap();
        assert_eq!(select.id, Some("id".to_string()));
        assert_eq!(select.name, "name");
        assert_eq!(select.color, Some(SelectColor::Blue));
    }

    #[test]
    fn serialize_select() {
        let select = Select {
            id: Some("id".to_string()),
            name: "name".to_string(),
            color: Some(SelectColor::Blue),
        };

        let json = serde_json::to_string(&select).expect("serialization failed");

        let expected = r#"{"id":"id","name":"name","color":"blue"}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn deserialize_select_group() {
        let json = r#"{
            "id": "id",
            "name": "name",
            "color": "blue",
            "option_ids": ["option_id"]
        }"#;
        let select_group: SelectGroup = serde_json::from_str(json).unwrap();
        assert_eq!(select_group.id, "id");
        assert_eq!(select_group.name, "name");
        assert_eq!(select_group.color, SelectColor::Blue);
        assert_eq!(select_group.option_ids, vec!["option_id"]);
    }

    #[test]
    fn check_try_from_color() {
        let cases = vec![
            ("blue", Some(SelectColor::Blue)),
            ("brown", Some(SelectColor::Brown)),
            ("gray", Some(SelectColor::Gray)),
            ("green", Some(SelectColor::Green)),
            ("orange", Some(SelectColor::Orange)),
            ("pink", Some(SelectColor::Pink)),
            ("purple", Some(SelectColor::Purple)),
            ("red", Some(SelectColor::Red)),
            ("yellow", Some(SelectColor::Yellow)),
            ("default", Some(SelectColor::Default)),
            ("invalid_color", None),
        ];

        for (input, expected) in cases {
            match SelectColor::from_str(input) {
                Ok(color) => assert_eq!(Some(color), expected, "Input: {}", input),
                Err(_) => assert!(expected.is_none(), "Expected None for input: {}", input),
            }
        }
    }
}
