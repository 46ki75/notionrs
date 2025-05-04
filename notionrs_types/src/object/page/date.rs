use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#date>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"date"`
/// - `$.['*'].date`: If the value is blank, it will be `null`.
/// - `$.['*'].date.start`: A date, with an optional time.
/// - `$.['*'].date.end`: A string representing the end of a date range.
///   If the value is null, then the date value is not a range.
/// - `$.['*'].date.time_zone`: Always `null`. The time zone is already included in the formats of start and end times.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example date page property value
///
/// ```json
/// {
///   "Date": {
///     "id": "w%5E%7DO",
///     "type": "date",
///     "date": {
///       "start": "2024-04-04T00:00:00.000+02:00",
///       "end": null,
///       "time_zone": null
///     }
///   }
/// }
/// ```
///
/// When the value is not set:
///
/// ```json
/// {
///   "Date": {
///     "id": "w%5E%7DO",
///     "type": "date",
///     "date": null
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct PageDateProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// If the value is blank, it will be `null`.
    pub date: Option<PageDatePropertyParameter>,
}

/// If the value is blank, it will be an empty object.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct PageDatePropertyParameter {
    /// A date, with an optional time.
    pub start: Option<crate::object::date::DateOrDateTime>,

    /// A string representing the end of a date range.
    /// If the value is null, then the date value is not a range.
    pub end: Option<crate::object::date::DateOrDateTime>,

    /// Always `null`. The time zone is already included in the formats of start and end times.
    #[serde(skip_deserializing)]
    pub time_zone: Option<String>,
}

impl PageDateProperty {
    pub fn start(&mut self, start: crate::object::date::DateOrDateTime) -> &mut Self {
        match &mut self.date {
            Some(date) => date.start = Some(start),
            None => {
                self.date = Some(PageDatePropertyParameter {
                    start: Some(start),
                    ..Default::default()
                });
            }
        }
        self
    }

    pub fn end(&mut self, end: crate::object::date::DateOrDateTime) -> &mut Self {
        match &mut self.date {
            Some(date) => date.end = Some(end),
            None => {
                self.date = Some(PageDatePropertyParameter {
                    end: Some(end),
                    ..Default::default()
                });
            }
        }
        self
    }
}

impl From<crate::object::date::DateOrDateTime> for PageDateProperty {
    fn from(value: crate::object::date::DateOrDateTime) -> Self {
        Self {
            id: None,
            date: Some(PageDatePropertyParameter {
                start: Some(value),
                ..Default::default()
            }),
        }
    }
}

impl std::fmt::Display for PageDateProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(date) = &self.date {
            write!(f, "{}", date)
        } else {
            write!(f, "")
        }
    }
}

impl std::fmt::Display for PageDatePropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.start {
            Some(start) => write!(f, "{}", start),
            None => write!(f, ""),
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
    use super::PageDateProperty;

    #[test]
    fn deserialize_page_date_property() {
        let json_data = r#"
        {
            "Date": {
                "id": "w%5E%7DO",
                "type": "date",
                "date": {
                    "start": "2024-04-04T00:00:00.000+02:00",
                    "end": null,
                    "time_zone": null
                }
            }
        }
        "#;

        let date_map =
            serde_json::from_str::<std::collections::HashMap<String, PageDateProperty>>(json_data)
                .unwrap();

        let date = date_map.get("Date").unwrap();

        assert_eq!(date.id, Some("w%5E%7DO".to_string()));

        if let Some(property) = &date.date {
            let expected_start = time::OffsetDateTime::parse(
                "2024-04-04T00:00:00.000+02:00",
                &time::format_description::well_known::Rfc3339,
            )
            .unwrap();
            assert_eq!(
                match property.start.unwrap() {
                    crate::object::date::DateOrDateTime::Date(_) => panic!(),
                    crate::object::date::DateOrDateTime::DateTime(offset_date_time) =>
                        offset_date_time,
                },
                expected_start
            );
            assert_eq!(property.end, None);
            assert_eq!(property.time_zone, None);
        }
    }
}
