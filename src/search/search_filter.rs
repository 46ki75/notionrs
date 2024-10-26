use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/post-search>
/// A set of criteria, value and property keys,
/// that limits the results to either only pages or only databases.
/// Possible value values are "page" or "database".
/// The only supported property value is "object".
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct SearchFilter {
    /// The value of the property to filter the results by.
    /// Possible values for object type include page or database.
    /// Limitation: Currently the only filter allowed is object
    /// which will filter by type of object (either page or database)
    pub value: SearchFilterType,

    /// Always `"object"`
    ///
    /// The name of the property to filter by.
    /// Currently the only property you can filter by is the object type.
    /// Possible values include object.
    /// Limitation: Currently the only filter allowed is object
    /// which will filter by type of object (either page or database)
    pub property: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchFilterType {
    #[default]
    Page,
    Database,
}
