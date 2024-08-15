use serde::{Deserialize, Serialize};

// # --------------------------------------------------------------------------------
//
// Filter
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Filter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Filter>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
}

// # --------------------------------------------------------------------------------
//
// Enum
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    Checkbox(CheckboxFilter),
    Date(DateFilter),
    Files(FilesFilter),
    // TODO: implement formula
    // TODO: implement multi_select
    // TODO: implement number
    // TODO: implement people
    // TODO: implement phone_number
    // TODO: implement relation
    // TODO: implement rich_text
    // TODO: implement select
    // TODO: implement status
    // TODO: implement timestamp
    // TODO: implement ID
}

// # --------------------------------------------------------------------------------
//
// Checkbox https://developers.notion.com/reference/post-database-query-filter#checkbox
//
// # --------------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy)]
pub struct CheckboxFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// Date https://developers.notion.com/reference/post-database-query-filter#date
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub next_month: Option<()>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_week: Option<()>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_year: Option<()>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_or_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_month: Option<()>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_week: Option<()>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_year: Option<()>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub this_week: Option<()>,
}

// # --------------------------------------------------------------------------------
//
// Files https://developers.notion.com/reference/post-database-query-filter#files
//
// # --------------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy)]
pub struct FilesFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_empty: Option<bool>,
}

// # --------------------------------------------------------------------------------
//
// formula
//
// # --------------------------------------------------------------------------------

// TODO: implement formula

// # --------------------------------------------------------------------------------
//
// multi_select
//
// # --------------------------------------------------------------------------------

// TODO: implement multi_select

// # --------------------------------------------------------------------------------
//
// number
//
// # --------------------------------------------------------------------------------

// TODO: implement number

// # --------------------------------------------------------------------------------
//
// people
//
// # --------------------------------------------------------------------------------

// TODO: implement people

// # --------------------------------------------------------------------------------
//
// phone_number
//
// # --------------------------------------------------------------------------------

// TODO: implement phone_number

// # --------------------------------------------------------------------------------
//
// relation
//
// # --------------------------------------------------------------------------------

// TODO: implement relation

// # --------------------------------------------------------------------------------
//
// rich_text
//
// # --------------------------------------------------------------------------------

// TODO: implement rich_text

// # --------------------------------------------------------------------------------
//
// select
//
// # --------------------------------------------------------------------------------

// TODO: implement select

// # --------------------------------------------------------------------------------
//
// status
//
// # --------------------------------------------------------------------------------

// TODO: implement status

// # --------------------------------------------------------------------------------
//
// timestamp
//
// # --------------------------------------------------------------------------------

// TODO: implement timestamp

// # --------------------------------------------------------------------------------
//
// ID
//
// # --------------------------------------------------------------------------------

// TODO: implement ID

// # --------------------------------------------------------------------------------
//
// Filter
//
// # --------------------------------------------------------------------------------

impl Filter {
    pub fn and(filters: Vec<Filter>) -> Self {
        Filter {
            and: Some(filters),
            or: None,
            property: None,
            condition: None,
        }
    }

    pub fn or(filters: Vec<Filter>) -> Self {
        Filter {
            and: None,
            or: Some(filters),
            property: None,
            condition: None,
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Checkbox
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
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Date
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
            condition: Some(Condition::Date(DateFilter {
                after: Some(date.as_ref().to_string()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                before: Some(date.as_ref().to_string()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                equals: Some(date.as_ref().to_string()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                is_empty: Some(true),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                is_not_empty: Some(true),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                next_month: Some(()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                next_week: Some(()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                next_year: Some(()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                on_or_after: Some(date.as_ref().to_string()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                on_or_before: Some(date.as_ref().to_string()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                past_month: Some(()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                past_week: Some(()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                past_year: Some(()),
                ..Default::default()
            })),
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
            condition: Some(Condition::Date(DateFilter {
                this_week: Some(()),
                ..Default::default()
            })),
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Files
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
        }
    }

    // TODO: implement formula
    // TODO: implement multi_select
    // TODO: implement number
    // TODO: implement people
    // TODO: implement phone_number
    // TODO: implement relation
    // TODO: implement rich_text
    // TODO: implement select
    // TODO: implement status
    // TODO: implement timestamp
    // TODO: implement ID
}
