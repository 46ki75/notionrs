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
    DataSource,
}

impl SearchFilter {
    pub fn page() -> Self {
        SearchFilter {
            value: SearchFilterType::Page,
            property: String::from("object"),
        }
    }

    pub fn database() -> Self {
        SearchFilter {
            value: SearchFilterType::DataSource,
            property: String::from("object"),
        }
    }
}

/// A search filter that can restrict results by object type, trash status, or both.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum SearchFilterRequest {
    /// Restricts search results to pages or data sources, optionally in the trash.
    Object {
        /// The value of the property to filter the results by.
        value: SearchFilterType,

        /// Always `"object"`.
        property: String,

        /// Whether to return only trashed objects.
        #[serde(skip_serializing_if = "Option::is_none")]
        in_trash: Option<bool>,
    },

    /// Restricts search results to trashed or live objects without limiting their type.
    InTrash {
        /// Whether to return only trashed objects.
        in_trash: bool,
    },
}

impl SearchFilterRequest {
    pub fn page() -> Self {
        Self::Object {
            value: SearchFilterType::Page,
            property: String::from("object"),
            in_trash: None,
        }
    }

    pub fn database() -> Self {
        Self::Object {
            value: SearchFilterType::DataSource,
            property: String::from("object"),
            in_trash: None,
        }
    }

    /// Restricts search results to trashed or live objects without limiting their type.
    pub fn trash(in_trash: bool) -> Self {
        Self::InTrash { in_trash }
    }

    /// Restricts search results to trashed or live objects.
    pub fn in_trash(self, in_trash: bool) -> Self {
        match self {
            Self::Object {
                value, property, ..
            } => Self::Object {
                value,
                property,
                in_trash: Some(in_trash),
            },
            Self::InTrash { .. } => Self::InTrash { in_trash },
        }
    }
}

impl Default for SearchFilterRequest {
    fn default() -> Self {
        Self::page()
    }
}

/// <https://developers.notion.com/reference/post-search>
/// A set of criteria that orders the search results — either by a timestamp
/// (with a direction) or by relevance to the search query.
/// The only supported timestamp value is "last_edited_time".
/// Supported direction values are "ascending" and "descending".
/// If sort is not provided, then the most recently edited results are returned first.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum SearchSort {
    /// Sort by a timestamp.
    Timestamp {
        /// `SearchSortDirection::Ascending` or `SearchSortDirection:Descending`
        direction: SearchSortDirection,

        /// Always `"last_edited_time"`
        ///
        /// The name of the timestamp to sort against. Possible values include last_edited_time.
        timestamp: String,
    },

    /// Sort by relevance to the search query. Added in `notion-sdk-js` v5.23.0.
    Relevance {
        /// Always `"relevance"`
        property: String,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchSortDirection {
    #[default]
    Ascending,
    Descending,
}

impl Default for SearchSort {
    fn default() -> Self {
        SearchSort::asc()
    }
}

impl SearchSort {
    pub fn asc() -> Self {
        SearchSort::Timestamp {
            direction: SearchSortDirection::Ascending,
            timestamp: String::from("last_edited_time"),
        }
    }

    pub fn desc() -> Self {
        SearchSort::Timestamp {
            direction: SearchSortDirection::Descending,
            timestamp: String::from("last_edited_time"),
        }
    }

    /// Sort by relevance to the search query.
    pub fn relevance() -> Self {
        SearchSort::Relevance {
            property: String::from("relevance"),
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn search_filter_and_sort() {
        let page = SearchFilter::page();
        assert_eq!(page.value, SearchFilterType::Page);
        assert_eq!(page.property, "object");

        let db = SearchFilter::database();
        assert_eq!(db.value, SearchFilterType::DataSource);

        let trashed_pages = SearchFilterRequest::page().in_trash(true);
        assert_eq!(
            serde_json::to_value(&trashed_pages).unwrap(),
            serde_json::json!({"property": "object", "value": "page", "in_trash": true})
        );

        let live_objects = SearchFilterRequest::trash(false);
        assert_eq!(
            serde_json::to_value(&live_objects).unwrap(),
            serde_json::json!({"in_trash": false})
        );

        let asc = SearchSort::asc();
        match &asc {
            SearchSort::Timestamp {
                direction,
                timestamp,
            } => {
                assert_eq!(*direction, SearchSortDirection::Ascending);
                assert_eq!(timestamp, "last_edited_time");
            }
            _ => panic!("Expected Timestamp variant"),
        }

        let desc = SearchSort::desc();
        match &desc {
            SearchSort::Timestamp { direction, .. } => {
                assert_eq!(*direction, SearchSortDirection::Descending);
            }
            _ => panic!("Expected Timestamp variant"),
        }

        let relevance = SearchSort::relevance();
        match &relevance {
            SearchSort::Relevance { property } => assert_eq!(property, "relevance"),
            _ => panic!("Expected Relevance variant"),
        }

        let json = serde_json::to_string(&page).unwrap();
        let _: SearchFilter = serde_json::from_str(&json).unwrap();
        let json = serde_json::to_string(&live_objects).unwrap();
        let deserialized_live_objects: SearchFilterRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized_live_objects, live_objects);
        let json = serde_json::to_string(&asc).unwrap();
        let _: SearchSort = serde_json::from_str(&json).unwrap();
        let json = serde_json::to_string(&relevance).unwrap();
        let deserialized_relevance: SearchSort = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized_relevance, relevance);

        let _ = SearchFilter::default();
        let _ = SearchFilterRequest::default();
        let _ = SearchSort::default();
        assert_eq!(SearchFilterType::default(), SearchFilterType::Page);
        assert_eq!(
            SearchSortDirection::default(),
            SearchSortDirection::Ascending
        );
    }
}
