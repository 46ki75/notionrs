use serde::{Deserialize, Serialize};

// # --------------------------------------------------------------------------------
//
// Filter
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DateFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_before: Option<String>,

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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]

pub struct PeopleFilter {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_before: Option<String>,

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
            or: None,
            property: None,
            condition: None,
            timestamp: None,
        }
    }

    pub fn or(filters: Vec<Filter>) -> Self {
        Filter {
            and: None,
            or: Some(filters.into_iter().map(Box::new).collect()),
            property: None,
            condition: None,
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Checkbox(CheckboxFilter { equals: Some(true) })),
            timestamp: None,
        }
    }

    /// Return the records where the checkbox is NOT checked.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn checkbox_is_not_checked<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Checkbox(CheckboxFilter {
                equals: Some(false),
            })),
            timestamp: None,
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
    /// - `date`: ISO 8601 date
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn date_after<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                after: Some(date.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where the date property value is before the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: The value to compare the date property value against. (ISO 8601 date)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn date_before<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                before: Some(date.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where the date property value is the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: The value to compare the date property value against. (ISO 8601 date)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn date_equals<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                equals: Some(date.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where the date property value contains no data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                is_empty: Some(true),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where the date property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                is_not_empty: Some(true),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the next month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_next_month<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                next_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the next week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_next_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                next_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the next year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_next_year<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                next_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where the date property value
    /// is on or after the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: The value to compare the date property value against. (ISO 8601 date)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn date_on_or_after<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                on_or_after: Some(date.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where the date property value is on or before the provided date.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `date`: The value to compare the date property value against. (ISO 8601 date)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn date_on_or_before<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                on_or_before: Some(date.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the past month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_past_month<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                past_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the past week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_past_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                past_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is within the past year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_past_year<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                past_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// A filter that limits the results to database entries
    /// where the date property value is this week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn date_this_week<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Date(Box::new(DateFilter {
                this_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Files(FilesFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    /// Returns all entries with a populated files property value.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn files_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Files(FilesFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Formula <https://developers.notion.com/reference/post-database-query-filter#formula>
    //
    // # --------------------------------------------------------------------------------

    pub fn formula_number_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Formula(Box::new(FormulaFilter {
                number: Some(NumberFilter {
                    is_empty: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                contains: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                does_not_contain: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::MultiSelect(MultiSelectFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                does_not_equal: Some(number.into()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                equals: Some(number.into()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                greater_than: Some(number.into()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                greater_than_or_equal_to: Some(number.into()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                less_than: Some(number.into()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Number(NumberFilter {
                less_than_or_equal_to: Some(number.into()),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // people <https://developers.notion.com/reference/post-database-query-filter#people>
    //
    // # --------------------------------------------------------------------------------

    /// Returns database entries where the people property value contains the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `id`: The value to compare the people property value against.
    pub fn people_contains<S, T>(property_name: S, id: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                contains: Some(id.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    /// Returns database entries where the people property value does not contain the provided string.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `id`: The value to compare the people property value against.
    pub fn people_does_not_contain<S, T>(property_name: S, id: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                does_not_contain: Some(id.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::People(PeopleFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                contains: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    pub fn phone_number_does_not_contain<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                does_not_contain: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    pub fn phone_number_does_not_equal<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                does_not_equal: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    pub fn phone_number_ends_with<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                ends_with: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    pub fn phone_number_equals<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                equals: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    pub fn phone_number_is_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    pub fn phone_number_is_not_empty<S>(property_name: S) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
        }
    }

    pub fn phone_number_starts_with<S, T>(property_name: S, phone_number: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::PhoneNumber(PhoneNumberFilter {
                starts_with: Some(phone_number.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                contains: Some(uuid.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                does_not_contain: Some(uuid.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Relation(RelationFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                contains: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                does_not_contain: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                does_not_equal: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                ends_with: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                equals: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::RichText(RichTextFilter {
                starts_with: Some(text.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Rollup(Box::new(RollupFilter {
                any: Some(Box::new(filter)),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where every rollup property value matches the provided criteria.
    pub fn rollup_every<S>(property_name: S, filter: Filter) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Rollup(Box::new(RollupFilter {
                every: Some(Box::new(filter)),
                ..Default::default()
            }))),
            timestamp: None,
        }
    }

    /// Returns database entries where no rollup property value matches the provided criteria.
    pub fn rollup_none<S>(property_name: S, filter: Filter) -> Self
    where
        S: AsRef<str>,
    {
        Filter {
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Rollup(Box::new(RollupFilter {
                none: Some(Box::new(filter)),
                ..Default::default()
            }))),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                does_not_equal: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                equals: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Select(SelectFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                does_not_equal: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                equals: Some(option_name.as_ref().to_string()),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::Status(StatusFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
            timestamp: None,
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
    /// - `timestamp`: ISO 8601 timestamp
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn timestamp_after<T: AsRef<str>>(timestamp: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                after: Some(timestamp.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// Returns database entries where the timestamp property value is before the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: The value to compare the timestamp property value against. (ISO 8601 timestamp)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn timestamp_before<T: AsRef<str>>(timestamp: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                before: Some(timestamp.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// Returns database entries where the timestamp property value is the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: The value to compare the timestamp property value against. (ISO 8601 timestamp)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn timestamp_equals<T: AsRef<str>>(timestamp: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                equals: Some(timestamp.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// Returns database entries where the timestamp property value contains no data.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_is_empty() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                is_empty: Some(true),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// Returns database entries where the timestamp property value is not empty.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_is_not_empty() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                is_not_empty: Some(true),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the next month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_next_month() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                next_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the next week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_next_week() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                next_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the next year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_next_year() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                next_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// Returns database entries where the timestamp property value
    /// is on or after the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: The value to compare the timestamp property value against. (ISO 8601 timestamp)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn timestamp_on_or_after<T: AsRef<str>>(timestamp: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                on_or_after: Some(timestamp.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// Returns database entries where the timestamp property value is on or before the provided timestamp.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    /// - `timestamp`: The value to compare the timestamp property value against. (ISO 8601 timestamp)
    ///   - e.g.) `"2021-05-10"`, `"2021-05-10T12:00:00"`, `"2021-10-15T12:00:00-07:00"`
    pub fn timestamp_on_or_before<T: AsRef<str>>(timestamp: T) -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                on_or_before: Some(timestamp.as_ref().to_string()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the past month.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_past_month() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                past_month: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the past week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_past_week() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                past_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is within the past year.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_past_year() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                past_year: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
        }
    }

    /// A filter that limits the results to database entries
    /// where the timestamp property value is this week.
    ///
    /// - `property_name`: Property Name (Column Name) in Notion Database
    pub fn timestamp_this_week() -> Self {
        Filter {
            and: None,
            or: None,
            property: None,
            condition: Some(Condition::Timestamp(Box::new(TimestampFilter {
                this_week: Some(std::collections::HashMap::new()),
                ..Default::default()
            }))),
            timestamp: Some("created_time".to_string()),
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                does_not_equal: Some(unique_id),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                equals: Some(unique_id),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                greater_than: Some(unique_id),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                greater_than_or_equal_to: Some(unique_id),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                less_than: Some(unique_id),
                ..Default::default()
            })),
            timestamp: None,
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
            and: None,
            or: None,
            property: Some(property_name.as_ref().to_string()),
            condition: Some(Condition::UniqueId(UniqueIdFilter {
                less_than_or_equal_to: Some(unique_id),
                ..Default::default()
            })),
            timestamp: None,
        }
    }
}
