use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#multi-select>
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
///   "Multi-select": {
///     "id": "_bnY",
///     "type": "multi_select",
///     "multi_select": [
///       {
///         "id": "959ba3e3-5a64-4ee6-864b-9e94ddc024d5",
///         "name": "HTML",
///         "color": "orange"
///       },
///       {
///         "id": "f22b05c9-0225-4dee-b25b-db7e63a47e0b",
///         "name": "CSS",
///         "color": "blue"
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageMultiSelectProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub multi_select: Vec<crate::others::select::Select>,
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
