use serde::{Deserialize, Serialize};

use crate::user::User;

/// <https://developers.notion.com/reference/page-property-values#people>
///
/// Example people page property value
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
    pub id: String,
    pub people: Vec<User>,
}
