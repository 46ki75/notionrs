use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#date>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"__________"` // TODO: documentation replace placeholder
/// - `$.['*'].__________`: // TODO: documentation
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example __________ page property value // TODO: documentation replace placeholder
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
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub date: Option<PageDatePropertyParameter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageDatePropertyParameter {
    start: Option<String>,
    end: Option<String>,
    time_zone: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // TODO: test
}
