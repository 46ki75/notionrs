use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#people>
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
///   "Person": {
///     "type": "people",
///     "id": "pAoV",
///     "people": [
///       {
///         "object": "user",
///         "id": "4050d499-9586-4352-85a2-d4cb55a68200",
///         "name": "46ki75",
///         "avatar_url": null,
///         "type": "person",
///         "person": { "email": "46ki75@example.com" }
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PagePeopleProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub people: Vec<crate::user::User>,
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
