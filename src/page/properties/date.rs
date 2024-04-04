use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#date
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
#[derive(Debug, Deserialize, Serialize)]
pub struct PageDateProperty {
    pub id: String,
    pub date: PageDatePropertyParameter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageDatePropertyParameter {
    start: String,
    end: Option<String>,
    time_zone: Option<String>,
}
