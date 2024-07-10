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

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub condition: Option<FilterExpression>,
}

// # --------------------------------------------------------------------------------
//
// FilterExpression
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterExpression {
    pub property: String,
    #[serde(flatten)]
    pub condition: Condition,
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
    // TODO: implement files
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

#[derive(Serialize, Deserialize, Debug)]
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
// FilterExpression
//
// # --------------------------------------------------------------------------------

impl FilterExpression {
    // # --------------------------------------------------------------------------------
    //
    // checkbox
    //
    // # --------------------------------------------------------------------------------

    pub fn checkbox_is_checked<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Checkbox(CheckboxFilter { equals: Some(true) }),
        }
    }

    pub fn checkbox_is_not_checked<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Checkbox(CheckboxFilter {
                equals: Some(false),
            }),
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // date
    //
    // # --------------------------------------------------------------------------------

    pub fn date_after<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                after: Some(date.as_ref().to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn date_before<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                before: Some(date.as_ref().to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn date_equals<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                equals: Some(date.as_ref().to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn date_is_empty<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                is_empty: Some(true),
                ..Default::default()
            }),
        }
    }

    pub fn date_is_not_empty<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                is_not_empty: Some(true),
                ..Default::default()
            }),
        }
    }

    pub fn date_next_month<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                next_month: Some(()),
                ..Default::default()
            }),
        }
    }

    pub fn date_next_week<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                next_week: Some(()),
                ..Default::default()
            }),
        }
    }

    pub fn date_next_year<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                next_year: Some(()),
                ..Default::default()
            }),
        }
    }

    pub fn date_on_or_after<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                on_or_after: Some(date.as_ref().to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn date_on_or_before<S: AsRef<str>, T: AsRef<str>>(property_name: S, date: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                on_or_before: Some(date.as_ref().to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn date_past_month<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                past_month: Some(()),
                ..Default::default()
            }),
        }
    }

    pub fn date_past_week<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                past_week: Some(()),
                ..Default::default()
            }),
        }
    }

    pub fn date_past_year<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                past_year: Some(()),
                ..Default::default()
            }),
        }
    }

    pub fn date_this_week<T: AsRef<str>>(property_name: T) -> Self {
        FilterExpression {
            property: property_name.as_ref().to_string(),
            condition: Condition::Date(DateFilter {
                this_week: Some(()),
                ..Default::default()
            }),
        }
    }

    // TODO: implement files
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
// Filter
//
// # --------------------------------------------------------------------------------

impl Filter {
    pub fn and(filters: Vec<Filter>) -> Self {
        Filter {
            and: Some(filters),
            or: None,
            condition: None,
        }
    }

    pub fn or(filters: Vec<Filter>) -> Self {
        Filter {
            and: None,
            or: Some(filters),
            condition: None,
        }
    }

    pub fn new(single_filter: FilterExpression) -> Self {
        Filter {
            and: None,
            or: None,
            condition: Some(single_filter),
        }
    }
}
