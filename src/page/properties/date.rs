use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#date>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"date"`
/// - `$.['*'].date`: If the value is blank, it will be `null`.
/// - `$.['*'].date.start`: A date, with an optional time.
/// - `$.['*'].date.end`: A string representing the end of a date range.
///                         If the value is null, then the date value is not a range.
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct PageDateProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: Option<String>,

    /// If the value is blank, it will be `null`.
    pub date: Option<PageDatePropertyParameter>,
}

/// If the value is blank, it will be an empty object.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct PageDatePropertyParameter {
    /// A date, with an optional time.
    start: String,

    /// A string representing the end of a date range.
    /// If the value is null, then the date value is not a range.
    end: Option<String>,

    /// Always `null`. The time zone is already included in the formats of start and end times.
    time_zone: Option<String>,
}

impl crate::ToPlainText for PageDateProperty {
    /// Convert PageDateProperty to a plain string
    fn to_plain_text(&self) -> String {
        if let Some(date) = &self.date {
            date.clone().start
        } else {
            String::new()
        }
    }
}

impl crate::ToPlainText for PageDatePropertyParameter {
    /// Convert PageDatePropertyParameter to a plain string
    fn to_plain_text(&self) -> String {
        self.start.clone()
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

        match &date.date {
            Some(property) => {
                assert_eq!(property.start, "2024-04-04T00:00:00.000+02:00");
                assert_eq!(property.end, None);
                assert_eq!(property.time_zone, None);
            }
            None => {}
        }
    }
}
