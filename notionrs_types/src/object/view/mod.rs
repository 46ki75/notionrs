use serde::{Deserialize, Serialize};

/// The view type.
///
/// <https://developers.notion.com/reference/retrieve-a-view>
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ViewType {
    Table,
    Board,
    List,
    Calendar,
    Timeline,
    Gallery,
    Form,
    Chart,
    Map,
    Dashboard,
}

impl std::fmt::Display for ViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_plain::to_string(self)
            .map_err(|_| std::fmt::Error)?
            .fmt(f)
    }
}

/// A full view object response.
///
/// <https://developers.notion.com/reference/retrieve-a-view>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ViewResponse {
    /// The ID of the view.
    pub id: String,

    /// The parent database of the view.
    pub parent: crate::object::parent::DatabaseParent,

    /// The name of the view.
    pub name: String,

    /// The view type.
    pub r#type: ViewType,

    /// The time when the view was created.
    pub created_time: String,

    /// The time when the view was last edited.
    pub last_edited_time: String,

    /// Canonical deep link to the view in Notion.
    pub url: String,

    /// The ID of the data source this view is scoped to, or null for dashboard views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,

    /// The user who created the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<crate::object::user::User>,

    /// The user who last edited the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_by: Option<crate::object::user::User>,

    /// The filter applied to this view (same shape as data source query filter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,

    /// The sorts applied to this view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<serde_json::Value>>,

    /// Quick filters pinned to the view's filter bar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_filters: Option<serde_json::Value>,

    /// View presentation configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,

    /// For dashboard widget views, the ID of the parent dashboard view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_view_id: Option<String>,
}

/// A partial view object response (returned in list operations, create, delete).
///
/// <https://developers.notion.com/reference/list-views>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PartialViewResponse {
    /// The ID of the view.
    pub id: String,

    /// The parent database of the view.
    pub parent: crate::object::parent::DatabaseParent,

    /// The view type.
    pub r#type: ViewType,
}

/// A view query response containing paginated page references.
///
/// <https://developers.notion.com/reference/create-a-view-query>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ViewQueryResponse {
    /// The query ID.
    pub id: String,

    /// The view this query was executed against.
    pub view_id: String,

    /// When the cached query results expire.
    pub expires_at: String,

    /// Total number of results in the query.
    pub total_count: u64,

    /// The page results for this page.
    pub results: Vec<ViewQueryPageReference>,

    /// Cursor for the next page of results.
    pub next_cursor: Option<String>,

    /// Whether there are more results.
    pub has_more: bool,

    /// Optional status indicating whether the result set is complete or was truncated
    /// by the server-side pagination depth limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_status: Option<crate::object::response::RequestStatus>,
}

/// A page reference within a view query result.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ViewQueryPageReference {
    /// The object type.
    pub object: String,

    /// The object ID.
    pub id: String,
}

/// Response for a deleted view query.
///
/// <https://developers.notion.com/reference/delete-a-view-query>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeletedViewQueryResponse {
    /// The ID of the deleted view query.
    pub id: String,

    /// Whether the view query was deleted.
    pub deleted: bool,
}

/// A view reference (used in list responses).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ViewReference {
    /// The ID of the view.
    pub id: String,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_view_response() {
        let json_data = r#"
        {
            "object": "view",
            "id": "abc12345-6789-0def-abcd-ef0123456789",
            "parent": {
                "type": "database_id",
                "database_id": "def12345-6789-0abc-def0-123456789abc"
            },
            "name": "Default View",
            "type": "table",
            "created_time": "2026-03-20T10:00:00.000Z",
            "last_edited_time": "2026-03-20T10:00:00.000Z",
            "url": "https://www.notion.so/abc123456789",
            "data_source_id": "src12345-6789-0abc-def0-123456789abc",
            "created_by": {
                "object": "user",
                "id": "usr12345-6789-0abc-def0-123456789abc"
            },
            "last_edited_by": {
                "object": "user",
                "id": "usr12345-6789-0abc-def0-123456789abc"
            },
            "filter": null,
            "sorts": null,
            "quick_filters": null,
            "configuration": {
                "type": "table",
                "properties": []
            }
        }
        "#;

        let view = serde_json::from_str::<ViewResponse>(json_data).unwrap();
        assert_eq!(view.name, "Default View");
        assert_eq!(view.r#type, ViewType::Table);
        assert_eq!(
            view.data_source_id,
            Some("src12345-6789-0abc-def0-123456789abc".to_string())
        );
    }

    #[test]
    fn deserialize_partial_view_response() {
        let json_data = r#"
        {
            "object": "view",
            "id": "abc12345-6789-0def-abcd-ef0123456789",
            "parent": {
                "type": "database_id",
                "database_id": "def12345-6789-0abc-def0-123456789abc"
            },
            "type": "board"
        }
        "#;

        let view = serde_json::from_str::<PartialViewResponse>(json_data).unwrap();
        assert_eq!(view.r#type, ViewType::Board);
    }

    #[test]
    fn deserialize_view_query_response() {
        let json_data = r#"
        {
            "object": "view_query",
            "id": "qry12345-6789-0abc-def0-123456789abc",
            "view_id": "view12345-6789-0abc-def0-123456789abc",
            "expires_at": "2026-03-20T11:00:00.000Z",
            "total_count": 2,
            "results": [
                {
                    "object": "page",
                    "id": "page12345-6789-0abc-def0-123456789abc"
                },
                {
                    "object": "page",
                    "id": "page23456-7890-1bcd-ef01-234567890bcd"
                }
            ],
            "next_cursor": null,
            "has_more": false
        }
        "#;

        let query = serde_json::from_str::<ViewQueryResponse>(json_data).unwrap();
        assert_eq!(query.total_count, 2);
        assert_eq!(query.results.len(), 2);
        assert!(!query.has_more);
        assert_eq!(query.request_status, None);
    }

    #[test]
    fn deserialize_view_query_response_with_request_status() {
        let json_data = r#"
        {
            "object": "view_query",
            "id": "qry12345-6789-0abc-def0-123456789abc",
            "view_id": "view12345-6789-0abc-def0-123456789abc",
            "expires_at": "2026-03-20T11:00:00.000Z",
            "total_count": 50,
            "results": [],
            "next_cursor": null,
            "has_more": false,
            "request_status": {
                "type": "incomplete",
                "incomplete_reason": "query_result_limit_reached"
            }
        }
        "#;

        let query = serde_json::from_str::<ViewQueryResponse>(json_data).unwrap();
        assert_eq!(
            query.request_status,
            Some(crate::object::response::RequestStatus::Incomplete {
                incomplete_reason: Some(
                    crate::object::response::IncompleteReason::QueryResultLimitReached
                ),
            })
        );
    }

    #[test]
    fn deserialize_deleted_view_query_response() {
        let json_data = r#"
        {
            "object": "view_query",
            "id": "qry12345-6789-0abc-def0-123456789abc",
            "deleted": true
        }
        "#;

        let deleted = serde_json::from_str::<DeletedViewQueryResponse>(json_data).unwrap();
        assert!(deleted.deleted);
    }

    #[test]
    fn view_type_display() {
        assert_eq!(ViewType::Table.to_string(), "table");
        assert_eq!(ViewType::Board.to_string(), "board");
        assert_eq!(ViewType::Dashboard.to_string(), "dashboard");
    }

    #[test]
    fn deserialize_view_response_all_view_types() {
        for view_type in [
            "table",
            "board",
            "list",
            "calendar",
            "timeline",
            "gallery",
            "form",
            "chart",
            "map",
            "dashboard",
        ] {
            let json_data = format!(
                r#"{{
                    "object": "view",
                    "id": "abc",
                    "parent": {{ "type": "database_id", "database_id": "def" }},
                    "name": "Test",
                    "type": "{}",
                    "created_time": "2026-01-01T00:00:00.000Z",
                    "last_edited_time": "2026-01-01T00:00:00.000Z",
                    "url": "https://www.notion.so/test"
                }}"#,
                view_type
            );

            let view = serde_json::from_str::<ViewResponse>(&json_data)
                .unwrap_or_else(|_| panic!("Failed to deserialize view type: {}", view_type));
            assert_eq!(view.r#type.to_string(), view_type);
        }
    }
}
