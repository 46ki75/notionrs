use serde::{Deserialize, Serialize};

// # --------------------------------------------------------------------------------
//
// Meeting Notes Filter
//
// # --------------------------------------------------------------------------------

/// Represents the property name that can be used for filtering meeting notes.
///
/// <https://developers.notion.com/reference/query-meeting-notes>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesPropertyName {
    Title,
    CreatedTime,
    LastEditedTime,
    CreatedBy,
    LastEditedBy,
    Attendees,
}

/// A combinator filter that combines multiple meeting notes filters using
/// `and` or `or` operators.
///
/// <https://developers.notion.com/reference/query-meeting-notes>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeetingNotesCombinatorFilter {
    /// The logical operator to combine filters.
    pub operator: MeetingNotesCombinatorOperator,

    /// Nested filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MeetingNotesFilterNode>>,
}

/// Logical operator for combining meeting notes filters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesCombinatorOperator {
    And,
    Or,
}

/// A node in the meeting notes filter tree; either a combinator or a property filter.
///
/// <https://developers.notion.com/reference/query-meeting-notes>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MeetingNotesFilterNode {
    Combinator(MeetingNotesCombinatorFilter),
    Property(MeetingNotesPropertyFilter),
}

/// A filter applied to a specific property of a meeting note.
///
/// <https://developers.notion.com/reference/query-meeting-notes>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeetingNotesPropertyFilter {
    /// The property to filter on.
    pub property: MeetingNotesPropertyName,
    /// The filter condition.
    pub filter: MeetingNotesPropertyFilterCondition,
}

/// Filter condition for a meeting notes property.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeetingNotesPropertyFilterCondition {
    /// The operator to apply.
    pub operator: String,
    /// Optional value for the filter operator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeetingNotesFilterValue>,
}

/// The value used in a meeting notes property filter.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MeetingNotesFilterValue {
    Text(MeetingNotesTextFilterValue),
    DatePoint(MeetingNotesDatePointFilterValue),
    DateRange(MeetingNotesDateRangeFilterValue),
    Person(Vec<MeetingNotesPersonFilterValue>),
}

/// Exact text value used for title filters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesTextFilterValue {
    pub r#type: MeetingNotesTextFilterValueType,
    pub value: String,
}

/// The type of a text filter value (always "exact").
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesTextFilterValueType {
    Exact,
}

/// A date point filter value (for `date_is`, `date_is_before`, etc.).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesDatePointFilterValue {
    #[serde(rename = "type")]
    pub value_type: MeetingNotesDateValueType,
    pub value: MeetingNotesDatePointValue,
}

/// A date range filter value (for `date_is_within`, `date_is_relative_to`).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesDateRangeFilterValue {
    #[serde(rename = "type")]
    pub value_type: MeetingNotesDateValueType,
    pub value: MeetingNotesDateRangeValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<MeetingNotesDateDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<MeetingNotesDateUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Type of a date filter value.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesDateValueType {
    Relative,
    Exact,
}

/// A date point value: either a string (e.g. `"today"`) or a structured date object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum MeetingNotesDatePointValue {
    String(String),
    Structured(MeetingNotesDateObject),
}

/// A date range value: either a string (e.g. `"this_week"`) or a date range object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum MeetingNotesDateRangeValue {
    String(String),
    Range(MeetingNotesDateRangeObject),
}

/// A structured date/datetime object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesDateObject {
    #[serde(rename = "type")]
    pub date_type: MeetingNotesDateObjectType,
    pub start_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// A date range object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesDateRangeObject {
    #[serde(rename = "type")]
    pub date_type: MeetingNotesDateRangeObjectType,
    pub start_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// Type of a date object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesDateObjectType {
    Date,
    Datetime,
}

/// Type of a date range object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesDateRangeObjectType {
    Daterange,
}

/// Direction for date range filters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesDateDirection {
    Past,
    Future,
}

/// Time unit for date range filters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesDateUnit {
    Day,
    Week,
    Month,
    Year,
}

/// A person filter value.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesPersonFilterValue {
    #[serde(rename = "type")]
    pub value_type: MeetingNotesPersonFilterValueType,
    pub value: MeetingNotesPersonValue,
}

/// Type of a person filter value (always "exact").
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesPersonFilterValueType {
    Exact,
}

/// A person value identifying a Notion user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesPersonValue {
    pub table: MeetingNotesPersonTable,
    pub id: String,
}

/// The table type for a person value.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesPersonTable {
    NotionUser,
}

// # --------------------------------------------------------------------------------
//
// Meeting Notes Sort
//
// # --------------------------------------------------------------------------------

/// Specifies the sort order for meeting notes query results.
///
/// <https://developers.notion.com/reference/query-meeting-notes>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeetingNotesSort {
    /// The property to sort by.
    pub property: MeetingNotesPropertyName,
    /// The sort direction.
    pub direction: MeetingNotesSortDirection,
}

/// Sort direction for meeting notes query results.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MeetingNotesSortDirection {
    Ascending,
    Descending,
}

// # --------------------------------------------------------------------------------
//
// Unit tests
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn serialize_combinator_filter() {
        let filter = MeetingNotesCombinatorFilter {
            operator: MeetingNotesCombinatorOperator::And,
            filters: Some(vec![MeetingNotesFilterNode::Property(
                MeetingNotesPropertyFilter {
                    property: MeetingNotesPropertyName::Title,
                    filter: MeetingNotesPropertyFilterCondition {
                        operator: "string_contains".to_string(),
                        value: Some(MeetingNotesFilterValue::Text(
                            MeetingNotesTextFilterValue {
                                r#type: MeetingNotesTextFilterValueType::Exact,
                                value: "standup".to_string(),
                            },
                        )),
                    },
                },
            )]),
        };

        let serialized = serde_json::to_string(&filter).unwrap();
        let deserialized: MeetingNotesCombinatorFilter =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(filter, deserialized);
    }

    #[test]
    fn serialize_sort() {
        let sort = MeetingNotesSort {
            property: MeetingNotesPropertyName::CreatedTime,
            direction: MeetingNotesSortDirection::Descending,
        };

        let serialized = serde_json::to_string(&sort).unwrap();
        assert!(serialized.contains("created_time"));
        assert!(serialized.contains("descending"));
    }

    #[test]
    fn serialize_empty_filter() {
        let filter = MeetingNotesPropertyFilter {
            property: MeetingNotesPropertyName::Attendees,
            filter: MeetingNotesPropertyFilterCondition {
                operator: "is_not_empty".to_string(),
                value: None,
            },
        };

        let serialized = serde_json::to_string(&filter).unwrap();
        assert!(serialized.contains("attendees"));
        assert!(serialized.contains("is_not_empty"));
        assert!(!serialized.contains("value"));
    }
}
