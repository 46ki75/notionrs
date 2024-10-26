use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/post-search>
/// A set of criteria, direction and timestamp keys, that orders the results.
/// The only supported timestamp value is "last_edited_time".
/// Supported direction values are "ascending" and "descending".
/// If sort is not provided, then the most recently edited results are returned first.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct SearchSort {
    /// `SearchSortDirection::Ascending` or `SearchSortDirection:Descending`
    pub direction: SearchSortDirection,

    /// Always `"last_edited_time"`
    ///
    ///  The name of the timestamp to sort against. Possible values include last_edited_time.
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchSortDirection {
    #[default]
    Ascending,
    Descending,
}

impl SearchSort {
    pub fn asc() -> Self {
        SearchSort {
            direction: SearchSortDirection::Ascending,
            timestamp: String::from("last_edited_time"),
        }
    }

    pub fn desc() -> Self {
        SearchSort {
            direction: SearchSortDirection::Descending,
            timestamp: String::from("last_edited_time"),
        }
    }
}
