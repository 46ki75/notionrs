use serde::{Deserialize, Serialize};

use crate::object::data_source::rollup::RollupFunction;

/// A calculated rollup page property value.
///
/// <https://developers.notion.com/reference/page-property-values#rollup>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageRollupProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The result of evaluating the rollup.
    pub rollup: Rollup,
}

impl std::fmt::Display for PageRollupProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rollup)
    }
}

/// The calculated value of a rollup property.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Rollup {
    Number(RollupNumber),
    Date(RollupDate),
    Array(RollupArray),
    Unsupported(RollupUnsupported),
    Incomplete(RollupIncomplete),
}

impl std::fmt::Display for Rollup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{}", value.number.unwrap_or(0.0)),
            Self::Date(value) => match &value.date {
                Some(date) => write!(f, "{date}"),
                None => write!(f, ""),
            },
            Self::Array(value) => {
                for item in &value.array {
                    if let RollupArrayItem::Property(property) = item {
                        write!(f, "{property}")?;
                    }
                }
                Ok(())
            }
            Self::Unsupported(_) | Self::Incomplete(_) => write!(f, ""),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RollupNumber {
    pub number: Option<f64>,
    pub function: RollupFunction,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RollupDate {
    pub date: Option<crate::object::page::date::PageDatePropertyParameter>,
    pub function: RollupFunction,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RollupArray {
    pub array: Vec<RollupArrayItem>,
    pub function: RollupFunction,
}

/// An item in an array rollup.
///
/// Page responses contain property values, while property-item responses use
/// empty placeholder objects.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum RollupArrayItem {
    Property(crate::object::page::PageProperty),
    Value(serde_json::Value),
}

/// A rollup value that Notion did not compute.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RollupUnsupported {
    pub unsupported: std::collections::HashMap<(), ()>,
    pub function: RollupFunction,
}

/// A partial rollup value returned while paginating property items.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RollupIncomplete {
    pub incomplete: std::collections::HashMap<(), ()>,
    pub function: RollupFunction,
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_unsupported_rollup() {
        let property: PageRollupProperty = serde_json::from_value(serde_json::json!({
            "id": "rollup-id",
            "rollup": {
                "type": "unsupported",
                "unsupported": {},
                "function": "sum"
            }
        }))
        .unwrap();

        assert!(matches!(property.rollup, Rollup::Unsupported(_)));
        assert_eq!(property.to_string(), "");
    }

    #[test]
    fn deserialize_rollup_value_variants() {
        let number: Rollup = serde_json::from_value(serde_json::json!({
            "type": "number",
            "number": 3.5,
            "function": "average"
        }))
        .unwrap();
        assert_eq!(number.to_string(), "3.5");

        let date: Rollup = serde_json::from_value(serde_json::json!({
            "type": "date",
            "date": {
                "start": "2026-08-05",
                "end": null,
                "time_zone": null
            },
            "function": "earliest_date"
        }))
        .unwrap();
        assert_eq!(date.to_string(), "2026-08-05");

        let incomplete: Rollup = serde_json::from_value(serde_json::json!({
            "type": "incomplete",
            "incomplete": {},
            "function": "sum"
        }))
        .unwrap();
        assert!(matches!(incomplete, Rollup::Incomplete(_)));

        let array: Rollup = serde_json::from_value(serde_json::json!({
            "type": "array",
            "array": [
                {
                    "type": "relation",
                    "relation": [{ "id": "page-id" }]
                },
                {}
            ],
            "function": "show_original"
        }))
        .unwrap();
        assert!(matches!(array, Rollup::Array(_)));
    }
}
