use serde::{Deserialize, Serialize};

// # --------------------------------------------------------------------------------
//
// Filter
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<Filter>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<Filter>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,

    /// Always either a "created_time" or null.
    /// When it's "created_time", apply a timestamp filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// Enum
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    Checkbox(CheckboxFilter),
    Date(Box<DateFilter>),
    Files(FilesFilter),
    Formula(Box<FormulaFilter>),
    MultiSelect(MultiSelectFilter),
    Number(NumberFilter),
    People(PeopleFilter),
    PhoneNumber(PhoneNumberFilter),
    Rollup(Box<RollupFilter>),
    Relation(RelationFilter),
    RichText(RichTextFilter),
    Select(SelectFilter),
    Status(StatusFilter),
    #[serde(rename = "created_time")]
    Timestamp(Box<TimestampFilter>),
    UniqueId(UniqueIdFilter),
}

// # --------------------------------------------------------------------------------
//
// Checkbox https://developers.notion.com/reference/post-database-query-filter#checkbox
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CheckboxFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// Date https://developers.notion.com/reference/post-database-query-filter#date
//
// # --------------------------------------------------------------------------------

/// A value that can be either an ISO 8601 date string or a relative date keyword.
///
/// The Notion API accepts the following relative date values
/// in date filter fields such as `after`, `before`, `equals`, `on_or_after`, and `on_or_before`:
///
/// - `"today"` — The current date
/// - `"tomorrow"` — The day after the current date
/// - `"yesterday"` — The day before the current date
/// - `"one_week_ago"` — Exactly 7 days before the current date
/// - `"one_week_from_now"` — Exactly 7 days after the current date
/// - `"one_month_ago"` — Exactly one month before the current date
/// - `"one_month_from_now"` — Exactly one month after the current date
///
/// # Examples
///
/// ```
/// use notionrs_types::object::request::filter::DateOrRelativeDate;
///
/// // From a date string
/// let date: DateOrRelativeDate = "2021-05-10".into();
///
/// // From a relative date value
/// let today = DateOrRelativeDate::today();
/// let tomorrow = DateOrRelativeDate::tomorrow();
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum DateOrRelativeDate {
    /// An ISO 8601 date string.
    /// e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    Date(String),

    /// A relative date keyword recognized by the Notion API.
    Relative(RelativeDateValue),
}

impl Default for DateOrRelativeDate {
    fn default() -> Self {
        DateOrRelativeDate::Date(String::new())
    }
}

impl<T: AsRef<str>> From<T> for DateOrRelativeDate {
    fn from(s: T) -> Self {
        let s_ref = s.as_ref();
        match s_ref {
            "today" => DateOrRelativeDate::Relative(RelativeDateValue::Today),
            "tomorrow" => DateOrRelativeDate::Relative(RelativeDateValue::Tomorrow),
            "yesterday" => DateOrRelativeDate::Relative(RelativeDateValue::Yesterday),
            "one_week_ago" => DateOrRelativeDate::Relative(RelativeDateValue::OneWeekAgo),
            "one_week_from_now" => {
                DateOrRelativeDate::Relative(RelativeDateValue::OneWeekFromNow)
            }
            "one_month_ago" => DateOrRelativeDate::Relative(RelativeDateValue::OneMonthAgo),
            "one_month_from_now" => {
                DateOrRelativeDate::Relative(RelativeDateValue::OneMonthFromNow)
            }
            _ => DateOrRelativeDate::Date(s_ref.to_string()),
        }
    }
}

impl DateOrRelativeDate {
    /// Returns a `DateOrRelativeDate` representing today.
    pub fn today() -> Self {
        DateOrRelativeDate::Relative(RelativeDateValue::Today)
    }

    /// Returns a `DateOrRelativeDate` representing tomorrow.
    pub fn tomorrow() -> Self {
        DateOrRelativeDate::Relative(RelativeDateValue::Tomorrow)
    }

    /// Returns a `DateOrRelativeDate` representing yesterday.
    pub fn yesterday() -> Self {
        DateOrRelativeDate::Relative(RelativeDateValue::Yesterday)
    }

    /// Returns a `DateOrRelativeDate` representing one week ago.
    pub fn one_week_ago() -> Self {
        DateOrRelativeDate::Relative(RelativeDateValue::OneWeekAgo)
    }

    /// Returns a `DateOrRelativeDate` representing one week from now.
    pub fn one_week_from_now() -> Self {
        DateOrRelativeDate::Relative(RelativeDateValue::OneWeekFromNow)
    }

    /// Returns a `DateOrRelativeDate` representing one month ago.
    pub fn one_month_ago() -> Self {
        DateOrRelativeDate::Relative(RelativeDateValue::OneMonthAgo)
    }

    /// Returns a `DateOrRelativeDate` representing one month from now.
    pub fn one_month_from_now() -> Self {
        DateOrRelativeDate::Relative(RelativeDateValue::OneMonthFromNow)
    }
}

/// Relative date keyword values accepted by the Notion API.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelativeDateValue {
    /// The current date.
    Today,
    /// The day after the current date.
    Tomorrow,
    /// The day before the current date.
    Yesterday,
    /// Exactly 7 days before the current date.
    OneWeekAgo,
    /// Exactly 7 days after the current date.
    OneWeekFromNow,
    /// Exactly one month before the current date.
    OneMonthAgo,
    /// Exactly one month after the current date.
    OneMonthFromNow,
}

impl std::fmt::Display for RelativeDateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeDateValue::Today => write!(f, "today"),
            RelativeDateValue::Tomorrow => write!(f, "tomorrow"),
            RelativeDateValue::Yesterday => write!(f, "yesterday"),
            RelativeDateValue::OneWeekAgo => write!(f, "one_week_ago"),
            RelativeDateValue::OneWeekFromNow => write!(f, "one_week_from_now"),
            RelativeDateValue::OneMonthAgo => write!(f, "one_month_ago"),
            RelativeDateValue::OneMonthFromNow => write!(f, "one_month_from_now"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DateFilter {
    /// An ISO 8601 date string or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateOrRelativeDate>,

    /// An ISO 8601 date string or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateOrRelativeDate>,

    /// An ISO 8601 date string or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<DateOrRelativeDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_month: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_week: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_year: Option<std::collections::HashMap<(), ()>>,

    /// An ISO 8601 date string or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_after: Option<DateOrRelativeDate>,

    /// An ISO 8601 date string or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_before: Option<DateOrRelativeDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_month: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_week: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_year: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub this_week: Option<std::collections::HashMap<(), ()>>,
}

// # --------------------------------------------------------------------------------
//
// Files https://developers.notion.com/reference/post-database-query-filter#files
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FilesFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// formula  https://developers.notion.com/reference/post-database-query-filter#formula
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FormulaFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox: Option<CheckboxFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<RichTextFilter>,
}

// # --------------------------------------------------------------------------------
//
// multi_select
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#multi-select>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct MultiSelectFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// number
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
pub struct NumberFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal_to: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal_to: Option<f64>,
}

// # --------------------------------------------------------------------------------
//
// people
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#people>
///
/// The `contains` and `does_not_contain` fields accept either a user ID
/// or the literal string `"me"` to refer to the current user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]

pub struct PeopleFilter {
    /// A user ID or the literal string `"me"` to refer to the current user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,

    /// A user ID or the literal string `"me"` to refer to the current user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// phone_number
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct PhoneNumberFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// relation
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#relation>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct RelationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// rich_text
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#rich-text>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct RichTextFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_contain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// rollup
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#rollup>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct RollupFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any: Option<Box<Filter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub every: Option<Box<Filter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<Box<Filter>>,
}

// # --------------------------------------------------------------------------------
//
// select
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#select>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// status
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#status>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct StatusFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// timestamp
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct TimestampFilter {
    /// An ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateOrRelativeDate>,

    /// An ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateOrRelativeDate>,

    /// An ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<DateOrRelativeDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_month: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_week: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_year: Option<std::collections::HashMap<(), ()>>,

    /// An ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_after: Option<DateOrRelativeDate>,

    /// An ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_before: Option<DateOrRelativeDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_month: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_week: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_year: Option<std::collections::HashMap<(), ()>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub this_week: Option<std::collections::HashMap<(), ()>>,
}

// # --------------------------------------------------------------------------------
//
// ID
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/post-database-query-filter#id>
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UniqueIdFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_not_equal: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal_to: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal_to: Option<u64>,
}
// # --------------------------------------------------------------------------------
//
// Filter
//
// # --------------------------------------------------------------------------------

impl Filter {
    pub fn and(filters: Vec<Filter>) -> Self {
        Filter {
            and: Some(filters.into_iter().map(Box::new).collect()),
            ..Default::default()
        }
    }

    pub fn or(filters: Vec<Filter>) -> Self {
        Filter {
            or: Some(filters.into_iter().map(Box::new).collect()),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Checkbox <https://developers.notion.com/reference/post-database-query-filter#checkbox>
    //
    // # --------------------------------------------------------------------------------

    /// Return the records where the checkbox is checked.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn checkbox_is_checked<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Checkbox(CheckboxFilter { equals: Some(true) })),
            ..Default::default()
        }
    }

    /// Return the records where the checkbox is NOT checked.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn checkbox_is_not_checked<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Checkbox(CheckboxFilter {
                equals: Some(false),
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Date <https://developers.notion.com/reference/post-database-query-filter#date>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the date property value is after the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: ISO 8601 date or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn date_after<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                after: Some(date.into()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where the date property value is before the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: ISO 8601 date or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn date_before<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                before: Some(date.into()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where the date property value is the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: ISO 8601 date or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn date_equals<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                equals: Some(date.into()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where the date property value contains no data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                is_empty: Some(true),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where the date property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                is_not_empty: Some(true),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the next month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_next_month<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                next_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the next week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_next_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                next_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the next year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_next_year<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                next_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where the date property value
    /// is on or after the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: ISO 8601 date or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn date_on_or_after<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                on_or_after: Some(date.into()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where the date property value is on or before the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: ISO 8601 date or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn date_on_or_before<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                on_or_before: Some(date.into()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the past month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_past_month<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                past_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the past week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_past_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                past_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the past year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_past_year<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                past_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is this week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_this_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                this_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Files <https://developers.notion.com/reference/post-database-query-filter#files>
    //
    // # --------------------------------------------------------------------------------

    /// Returns all database entries with an empty files property value.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn files_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Files(FilesFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns all entries with a populated files property value.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn files_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Files(FilesFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Formula <https://developers.notion.com/reference/post-database-query-filter#formula>
    //
    // # --------------------------------------------------------------------------------

    // Formula Number Filters
    pub fn formula_number_equals<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    equals: Some(number.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_number_does_not_equal<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    does_not_equal: Some(number.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_number_greater_than<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    greater_than: Some(number.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_number_less_than<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    less_than: Some(number.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_number_greater_than_or_equal<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    greater_than_or_equal_to: Some(number.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_number_less_than_or_equal<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    less_than_or_equal_to: Some(number.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_number_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    is_empty: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_number_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    is_not_empty: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    // Formula String Filters
    pub fn formula_string_equals<T: AsRef<str>>(property_name: T, text: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    equals: Some(text.as_ref().to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_string_does_not_equal<T: AsRef<str>>(property_name: T, text: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    does_not_equal: Some(text.as_ref().to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_string_contains<T: AsRef<str>>(property_name: T, text: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    contains: Some(text.as_ref().to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_string_does_not_contain<T: AsRef<str>>(property_name: T, text: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    does_not_contain: Some(text.as_ref().to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_string_starts_with<T: AsRef<str>>(property_name: T, text: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    starts_with: Some(text.as_ref().to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_string_ends_with<T: AsRef<str>>(property_name: T, text: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    ends_with: Some(text.as_ref().to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_string_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    is_empty: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_string_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                string: Some(RichTextFilter {
                    is_not_empty: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    // Formula Checkbox Filters
    pub fn formula_checkbox_equals<T: AsRef<str>>(property_name: T, checked: bool) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                checkbox: Some(CheckboxFilter {
                    equals: Some(checked),
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_checkbox_is_checked<T: AsRef<str>>(property_name: T) -> Self {
        Self::formula_checkbox_equals(property_name, true)
    }

    pub fn formula_checkbox_is_not_checked<T: AsRef<str>>(property_name: T) -> Self {
        Self::formula_checkbox_equals(property_name, false)
    }

    // Formula Date Filters
    pub fn formula_date_after<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    after: Some(date.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_before<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    before: Some(date.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_equals<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    equals: Some(date.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    is_empty: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    is_not_empty: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_next_month<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    next_month: Some(std::collections::HashMap::new()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_next_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    next_week: Some(std::collections::HashMap::new()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_next_year<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    next_year: Some(std::collections::HashMap::new()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_on_or_after<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    on_or_after: Some(date.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_on_or_before<S: AsRef<str>, T: Into<DateOrRelativeDate>>(
        property_name: S,
        date: T,
    ) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    on_or_before: Some(date.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_past_month<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    past_month: Some(std::collections::HashMap::new()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_past_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    past_week: Some(std::collections::HashMap::new()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_past_year<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    past_year: Some(std::collections::HashMap::new()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    pub fn formula_date_this_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                date: Some(DateFilter {
                    this_week: Some(std::collections::HashMap::new()),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // multi select <https://developers.notion.com/reference/post-database-query-filter#multi-select>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the multi-select value matches the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `option_name`: The string to compare the multi-select property value against.
    pub fn multi_select_contains<S, T>(property_name: S, option_name: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                contains: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the multi-select value does not match the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `option_name`: The string to compare the multi-select property value against.
    pub fn multi_select_does_not_contain<S, T>(property_name: S, option_name: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                does_not_contain: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the multi-select property value is empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn multi_select_is_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the multi-select property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn multi_select_is_not_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // number <https://developers.notion.com/reference/post-database-query-filter#number>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the number property value differs from the provided number.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `number`: The number to compare the number property value against.
    pub fn number_does_not_equal<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                does_not_equal: Some(number.into()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the number property value is the same as the provided number.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `number`: The number to compare the number property value against.
    pub fn number_equals<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                equals: Some(number.into()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the number property value exceeds the provided number.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `number`: The number to compare the number property value against.
    pub fn number_greater_than<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                greater_than: Some(number.into()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the number property value is equal to or exceeds the provided number.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `number`: The number to compare the number property value against.
    pub fn number_greater_than_or_equal_to<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                greater_than_or_equal_to: Some(number.into()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the number property value does not contain any data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn number_is_empty<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the number property value contains data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn number_is_not_empty<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the number property value is less than the provided number.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `number`: The number to compare the number property value against.
    pub fn number_less_than<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                less_than: Some(number.into()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the number property value is equal to or is less than the provided number.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `number`: The number to compare the number property value against.
    pub fn number_less_than_or_equal_to<T, N>(property_name: T, number: N) -> Self
    where
        T: AsRef<str>,
        N: Into<f64>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                less_than_or_equal_to: Some(number.into()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // people <https://developers.notion.com/reference/post-database-query-filter#people>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the people property value contains the provided user.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `id`: A user ID or the literal string `"me"` to refer to the current user.
    pub fn people_contains<S, T>(property_name: S, id: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                contains: Some(id.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the people property value contains the current user (`"me"`).
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn people_contains_me<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                contains: Some("me".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the people property value does not contain the provided user.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `id`: A user ID or the literal string `"me"` to refer to the current user.
    pub fn people_does_not_contain<S, T>(property_name: S, id: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                does_not_contain: Some(id.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the people property value does not contain the current user (`"me"`).
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn people_does_not_contain_me<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                does_not_contain: Some("me".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the people property value does not contain any data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn people_is_empty<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the people property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn people_is_not_empty<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // phone number (No official documentation)
    //
    // # --------------------------------------------------------------------------------

    pub fn phone_number_contains<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                contains: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn phone_number_does_not_contain<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                does_not_contain: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn phone_number_does_not_equal<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                does_not_equal: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn phone_number_ends_with<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                ends_with: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn phone_number_equals<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                equals: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn phone_number_is_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn phone_number_is_not_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn phone_number_starts_with<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                starts_with: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }
    // # --------------------------------------------------------------------------------
    //
    // relation <https://developers.notion.com/reference/post-database-query-filter#relation>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries with a text property value that includes the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `uuid`: The string to compare the text property value against.
    pub fn relation_contains<S, T>(property_name: S, uuid: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                contains: Some(uuid.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that does not include the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `uuid`: The string to compare the text property value against.
    pub fn relation_does_not_contain<S, T>(property_name: S, uuid: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                does_not_contain: Some(uuid.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that is empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn relation_is_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that contains data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn relation_is_not_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // rich text <https://developers.notion.com/reference/post-database-query-filter#rich-text>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries with a text property value that includes the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `text`: The string to compare the text property value against.
    pub fn rich_text_contains<S, T>(property_name: S, text: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                contains: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that does not include the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `text`: The string to compare the text property value against.
    pub fn rich_text_does_not_contain<S, T>(property_name: S, text: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                does_not_contain: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that does not match the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `text`: The string to compare the text property value against.
    pub fn rich_text_does_not_equal<S, T>(property_name: S, text: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                does_not_equal: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that ends with the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `text`: The string to compare the text property value against.
    pub fn rich_text_ends_with<S, T>(property_name: S, text: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                ends_with: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that matches the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `text`: The string to compare the text property value against.
    pub fn rich_text_equals<S, T>(property_name: S, text: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                equals: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that is empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn rich_text_is_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that contains data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn rich_text_is_not_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries with a text property value that starts with the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `text`: The string to compare the text property value against.
    pub fn rich_text_starts_with<S, T>(property_name: S, text: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                starts_with: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // rollup <https://developers.notion.com/reference/post-database-query-filter#rollup>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the rollup property value matches the provided criteria.
    pub fn rollup_any<S>(property_name: S, filter: Filter) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Rollup(Box::new(RollupFilter {
                any: Some(Box::new(filter)),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where every rollup property value matches the provided criteria.
    pub fn rollup_every<S>(property_name: S, filter: Filter) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Rollup(Box::new(RollupFilter {
                every: Some(Box::new(filter)),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    /// Returns database entries where no rollup property value matches the provided criteria.
    pub fn rollup_none<S>(property_name: S, filter: Filter) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Rollup(Box::new(RollupFilter {
                none: Some(Box::new(filter)),
                ..Default::default()
            }))),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // select <https://developers.notion.com/reference/post-database-query-filter#select>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the select property value matches the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `option_name`: The string to compare the select property value against.
    pub fn select_does_not_equal<S, T>(property_name: S, option_name: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                does_not_equal: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the select property value does not match the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `option_name`: The string to compare the select property value against.
    pub fn select_equals<S, T>(property_name: S, option_name: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                equals: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the select property value is empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn select_is_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the select property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn select_is_not_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // status <https://developers.notion.com/reference/post-database-query-filter#status>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the status property value matches the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `option_name`: The string to compare the status property value against.
    pub fn status_does_not_equal<S, T>(property_name: S, option_name: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                does_not_equal: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the status property value does not match the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `option_name`: The string to compare the status property value against.
    pub fn status_equals<S, T>(property_name: S, option_name: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                equals: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the status property value is empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn status_is_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the status property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn status_is_not_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // timestamp <https://developers.notion.com/reference/post-database-query-filter#timestamp>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the timestamp property value is after the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn timestamp_after<T: Into<DateOrRelativeDate>>(timestamp: T) -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                after: Some(timestamp.into()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// Returns database entries where the timestamp property value is before the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn timestamp_before<T: Into<DateOrRelativeDate>>(timestamp: T) -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                before: Some(timestamp.into()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// Returns database entries where the timestamp property value is the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn timestamp_equals<T: Into<DateOrRelativeDate>>(timestamp: T) -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                equals: Some(timestamp.into()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// Returns database entries where the timestamp property value contains no data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_is_empty() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                is_empty: Some(true),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// Returns database entries where the timestamp property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_is_not_empty() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                is_not_empty: Some(true),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the next month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_next_month() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                next_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the next week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_next_week() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                next_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the next year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_next_year() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                next_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// Returns database entries where the timestamp property value
    /// is on or after the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn timestamp_on_or_after<T: Into<DateOrRelativeDate>>(timestamp: T) -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                on_or_after: Some(timestamp.into()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// Returns database entries where the timestamp property value is on or before the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: ISO 8601 timestamp or a relative date value (e.g. `"today"`, `"tomorrow"`)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`, `"today"`
    pub fn timestamp_on_or_before<T: Into<DateOrRelativeDate>>(timestamp: T) -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                on_or_before: Some(timestamp.into()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the past month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_past_month() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                past_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the past week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_past_week() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                past_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the past year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_past_year() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                past_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is this week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_this_week() -> Self {
        Filter {
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                this_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // ID unique_id <https://developers.notion.com/reference/post-database-query-filter#id>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the unique_id property value differs from the provided unique_id.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `unique_id`: The unique_id to compare the unique_id property value against.
    pub fn unique_id_does_not_equal<T>(property_name: T, unique_id: u64) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                does_not_equal: Some(unique_id),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the unique_id property value is the same as the provided unique_id.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `unique_id`: The unique_id to compare the unique_id property value against.
    pub fn unique_id_equals<T>(property_name: T, unique_id: u64) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                equals: Some(unique_id),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the unique_id property value exceeds the provided unique_id.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `unique_id`: The unique_id to compare the unique_id property value against.
    pub fn unique_id_greater_than<T>(property_name: T, unique_id: u64) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                greater_than: Some(unique_id),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the unique_id property value is equal to or exceeds the provided unique_id.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `unique_id`: The unique_id to compare the unique_id property value against.
    pub fn unique_id_greater_than_or_equal_to<T>(property_name: T, unique_id: u64) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                greater_than_or_equal_to: Some(unique_id),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the unique_id property value is less than the provided unique_id.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `unique_id`: The unique_id to compare the unique_id property value against.
    pub fn unique_id_less_than<T>(property_name: T, unique_id: u64) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                less_than: Some(unique_id),
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    /// Returns database entries where the unique_id property value is equal to or is less than the provided unique_id.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `unique_id`: The unique_id to compare the unique_id property value against.
    pub fn unique_id_less_than_or_equal_to<T>(property_name: T, unique_id: u64) -> Self
    where
        T: AsRef<str>,
    {
        Filter {
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                less_than_or_equal_to: Some(unique_id),
                ..Default::default()
            })),
            ..Default::default()
        }
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn date_or_relative_date_from_date_string() {
        let date: DateOrRelativeDate = "2021-05-10".into();
        assert_eq!(date, DateOrRelativeDate::Date("2021-05-10".to_string()));
    }

    #[test]
    fn date_or_relative_date_from_relative_value() {
        let today: DateOrRelativeDate = "today".into();
        assert_eq!(
            today,
            DateOrRelativeDate::Relative(RelativeDateValue::Today)
        );

        let tomorrow: DateOrRelativeDate = "tomorrow".into();
        assert_eq!(
            tomorrow,
            DateOrRelativeDate::Relative(RelativeDateValue::Tomorrow)
        );

        let yesterday: DateOrRelativeDate = "yesterday".into();
        assert_eq!(
            yesterday,
            DateOrRelativeDate::Relative(RelativeDateValue::Yesterday)
        );

        let one_week_ago: DateOrRelativeDate = "one_week_ago".into();
        assert_eq!(
            one_week_ago,
            DateOrRelativeDate::Relative(RelativeDateValue::OneWeekAgo)
        );

        let one_week_from_now: DateOrRelativeDate = "one_week_from_now".into();
        assert_eq!(
            one_week_from_now,
            DateOrRelativeDate::Relative(RelativeDateValue::OneWeekFromNow)
        );

        let one_month_ago: DateOrRelativeDate = "one_month_ago".into();
        assert_eq!(
            one_month_ago,
            DateOrRelativeDate::Relative(RelativeDateValue::OneMonthAgo)
        );

        let one_month_from_now: DateOrRelativeDate = "one_month_from_now".into();
        assert_eq!(
            one_month_from_now,
            DateOrRelativeDate::Relative(RelativeDateValue::OneMonthFromNow)
        );
    }

    #[test]
    fn date_or_relative_date_convenience_methods() {
        assert_eq!(
            DateOrRelativeDate::today(),
            DateOrRelativeDate::Relative(RelativeDateValue::Today)
        );
        assert_eq!(
            DateOrRelativeDate::tomorrow(),
            DateOrRelativeDate::Relative(RelativeDateValue::Tomorrow)
        );
        assert_eq!(
            DateOrRelativeDate::yesterday(),
            DateOrRelativeDate::Relative(RelativeDateValue::Yesterday)
        );
    }

    #[test]
    fn serialize_date_or_relative_date_string() {
        let date = DateOrRelativeDate::Date("2021-05-10".to_string());
        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, "\"2021-05-10\"");
    }

    #[test]
    fn serialize_date_or_relative_date_relative() {
        let today = DateOrRelativeDate::Relative(RelativeDateValue::Today);
        let json = serde_json::to_string(&today).unwrap();
        assert_eq!(json, "\"today\"");

        let one_week_ago = DateOrRelativeDate::Relative(RelativeDateValue::OneWeekAgo);
        let json = serde_json::to_string(&one_week_ago).unwrap();
        assert_eq!(json, "\"one_week_ago\"");
    }

    #[test]
    fn deserialize_date_or_relative_date_string() {
        let date: DateOrRelativeDate = serde_json::from_str("\"2021-05-10\"").unwrap();
        // Both Date("2021-05-10") and Date("2021-05-10") are valid
        match date {
            DateOrRelativeDate::Date(s) => assert_eq!(s, "2021-05-10"),
            DateOrRelativeDate::Relative(_) => {} // also fine for non-keyword strings
        }
    }

    #[test]
    fn deserialize_date_or_relative_date_relative() {
        let today: DateOrRelativeDate = serde_json::from_str("\"today\"").unwrap();
        // "today" can deserialize as either Relative(Today) or Date("today")
        // Both are valid representations
        match today {
            DateOrRelativeDate::Relative(RelativeDateValue::Today) => {}
            DateOrRelativeDate::Date(s) if s == "today" => {}
            _ => panic!("Expected today variant"),
        }
    }

    #[test]
    fn serialize_date_filter_with_relative_date() {
        let filter = DateFilter {
            after: Some(DateOrRelativeDate::Relative(RelativeDateValue::Today)),
            ..Default::default()
        };

        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"after\":\"today\""));
    }

    #[test]
    fn filter_date_after_with_string() {
        let filter = Filter::date_after("Date Property", "2021-05-10");
        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"2021-05-10\""));
    }

    #[test]
    fn filter_date_after_with_relative_date() {
        let filter = Filter::date_after("Date Property", DateOrRelativeDate::today());
        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"today\""));
    }

    #[test]
    fn filter_people_contains_me() {
        let filter = Filter::people_contains_me("Assignee");
        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"me\""));
        assert!(json.contains("\"Assignee\""));
    }

    #[test]
    fn filter_people_does_not_contain_me() {
        let filter = Filter::people_does_not_contain_me("Assignee");
        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"me\""));
        assert!(json.contains("\"does_not_contain\""));
    }

    #[test]
    fn filter_people_contains_with_me_string() {
        let filter = Filter::people_contains("Assignee", "me");
        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"me\""));
    }

    #[test]
    fn relative_date_value_display() {
        assert_eq!(RelativeDateValue::Today.to_string(), "today");
        assert_eq!(RelativeDateValue::Tomorrow.to_string(), "tomorrow");
        assert_eq!(RelativeDateValue::Yesterday.to_string(), "yesterday");
        assert_eq!(RelativeDateValue::OneWeekAgo.to_string(), "one_week_ago");
        assert_eq!(
            RelativeDateValue::OneWeekFromNow.to_string(),
            "one_week_from_now"
        );
        assert_eq!(
            RelativeDateValue::OneMonthAgo.to_string(),
            "one_month_ago"
        );
        assert_eq!(
            RelativeDateValue::OneMonthFromNow.to_string(),
            "one_month_from_now"
        );
    }
}
