use serde::{Deserialize, Serialize};

/// Response from the `GET /v1/async_tasks/:id` endpoint.
///
/// <https://developers.notion.com/reference/retrieve-an-async-task>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AsyncTaskResponse {
    /// The task has been accepted but has not started running yet.
    Queued(AsyncTaskProgress),

    /// The task is currently running.
    Running(AsyncTaskProgress),

    /// The task failed and is being retried.
    Retrying(AsyncTaskProgress),

    /// The task completed successfully.
    Succeeded(AsyncTaskSucceeded),

    /// The task failed.
    Failed(AsyncTaskFailed),
}

/// Shared fields for a task in the `queued`, `running`, or `retrying` states.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct AsyncTaskProgress {
    /// Always `"async_task"`.
    pub object: String,

    /// The ID of the async task.
    pub id: String,

    /// URL to poll for the task's current status.
    pub status_url: String,

    /// ISO 8601 timestamp of when the task was created.
    pub created_time: String,

    /// The operation this task represents.
    pub operation: AsyncTaskOperation,

    /// Suggested number of seconds to wait before polling again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_after_seconds: Option<u64>,
}

/// Fields for a task that completed successfully.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct AsyncTaskSucceeded {
    /// Always `"async_task"`.
    pub object: String,

    /// The ID of the async task.
    pub id: String,

    /// URL to poll for the task's current status.
    pub status_url: String,

    /// ISO 8601 timestamp of when the task was created.
    pub created_time: String,

    /// The operation this task represents.
    pub operation: AsyncTaskOperation,

    /// The operation-specific result payload.
    pub result: std::collections::HashMap<String, serde_json::Value>,
}

/// Fields for a task that failed.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct AsyncTaskFailed {
    /// Always `"async_task"`.
    pub object: String,

    /// The ID of the async task.
    pub id: String,

    /// URL to poll for the task's current status.
    pub status_url: String,

    /// ISO 8601 timestamp of when the task was created.
    pub created_time: String,

    /// The operation this task represents.
    pub operation: AsyncTaskOperation,

    /// Details about why the task failed.
    pub error: AsyncTaskError,
}

/// The operation an async task is performing.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct AsyncTaskOperation {
    /// The surface that initiated the operation.
    pub surface: AsyncTaskOperationSurface,

    /// The name of the operation (e.g. `"create_page"`).
    pub name: String,
}

/// The surface that initiated an async task's operation.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AsyncTaskOperationSurface {
    /// The operation was initiated via the REST API.
    Rest,
    /// The operation was initiated via MCP.
    Mcp,
}

/// Error details for a failed async task.
///
/// <https://developers.notion.com/reference/errors>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct AsyncTaskError {
    /// The error code.
    pub code: AsyncTaskErrorCode,

    /// A human-readable description of the error.
    pub message: String,
}

/// Error code for a failed async task.
///
/// <https://developers.notion.com/reference/errors>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AsyncTaskErrorCode {
    /// The request body could not be decoded.
    InvalidJson,
    /// The request URL is not valid.
    InvalidRequestUrl,
    /// This request is not supported.
    InvalidRequest,
    /// The request is missing the `Notion-Version` header.
    MissingVersion,
    /// The body of the request is not valid.
    ValidationError,
    /// The bearer token is not valid.
    Unauthorized,
    /// Given the bearer token used, the client doesn't have permission to perform this operation.
    RestrictedResource,
    /// The resource does not exist.
    ObjectNotFound,
    /// The request exceeds the rate limit.
    RateLimited,
    /// Notion is temporarily overloaded and could not process the request.
    ServiceOverload,
    /// An unexpected error occurred on the Notion side.
    InternalServerError,
    /// Notion is unavailable.
    ServiceUnavailable,
    /// The request timed out at the gateway.
    GatewayTimeout,
    /// The transaction could not be completed, potentially due to a data collision.
    ConflictError,
    /// The request would return too many rows.
    RowLimitExceeded,
    /// An unknown error code.
    #[serde(untagged)]
    Unknown(String),
}

// # --------------------------------------------------------------------------------
//
// unit_tests
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_async_task_queued() {
        let json = r#"
        {
            "object": "async_task",
            "id": "task-id-123",
            "status": "queued",
            "status_url": "https://api.notion.com/v1/async_tasks/task-id-123",
            "created_time": "2026-01-01T00:00:00.000Z",
            "operation": { "surface": "rest", "name": "create_page" },
            "poll_after_seconds": 2
        }
        "#;

        let task: AsyncTaskResponse = serde_json::from_str(json).expect("Failed to deserialize");
        match task {
            AsyncTaskResponse::Queued(progress) => {
                assert_eq!(progress.id, "task-id-123");
                assert_eq!(progress.operation.name, "create_page");
                assert_eq!(progress.operation.surface, AsyncTaskOperationSurface::Rest);
                assert_eq!(progress.poll_after_seconds, Some(2));
            }
            _ => panic!("Expected Queued variant"),
        }
    }

    #[test]
    fn deserialize_async_task_running() {
        let json = r#"
        {
            "object": "async_task",
            "id": "task-id-123",
            "status": "running",
            "status_url": "https://api.notion.com/v1/async_tasks/task-id-123",
            "created_time": "2026-01-01T00:00:00.000Z",
            "operation": { "surface": "mcp", "name": "update_page_markdown" }
        }
        "#;

        let task: AsyncTaskResponse = serde_json::from_str(json).expect("Failed to deserialize");
        match task {
            AsyncTaskResponse::Running(progress) => {
                assert_eq!(progress.operation.surface, AsyncTaskOperationSurface::Mcp);
                assert!(progress.poll_after_seconds.is_none());
            }
            _ => panic!("Expected Running variant"),
        }
    }

    #[test]
    fn deserialize_async_task_retrying() {
        let json = r#"
        {
            "object": "async_task",
            "id": "task-id-123",
            "status": "retrying",
            "status_url": "https://api.notion.com/v1/async_tasks/task-id-123",
            "created_time": "2026-01-01T00:00:00.000Z",
            "operation": { "surface": "rest", "name": "create_page" }
        }
        "#;

        let task: AsyncTaskResponse = serde_json::from_str(json).expect("Failed to deserialize");
        assert!(matches!(task, AsyncTaskResponse::Retrying(_)));
    }

    #[test]
    fn deserialize_async_task_succeeded() {
        let json = r#"
        {
            "object": "async_task",
            "id": "task-id-123",
            "status": "succeeded",
            "status_url": "https://api.notion.com/v1/async_tasks/task-id-123",
            "created_time": "2026-01-01T00:00:00.000Z",
            "operation": { "surface": "rest", "name": "create_page" },
            "result": { "id": "page-id-456" }
        }
        "#;

        let task: AsyncTaskResponse = serde_json::from_str(json).expect("Failed to deserialize");
        match task {
            AsyncTaskResponse::Succeeded(succeeded) => {
                assert_eq!(
                    succeeded.result.get("id").and_then(|v| v.as_str()),
                    Some("page-id-456")
                );
            }
            _ => panic!("Expected Succeeded variant"),
        }
    }

    #[test]
    fn deserialize_async_task_failed() {
        let json = r#"
        {
            "object": "async_task",
            "id": "task-id-123",
            "status": "failed",
            "status_url": "https://api.notion.com/v1/async_tasks/task-id-123",
            "created_time": "2026-01-01T00:00:00.000Z",
            "operation": { "surface": "rest", "name": "create_page" },
            "error": { "code": "service_overload", "message": "Notion is overloaded." }
        }
        "#;

        let task: AsyncTaskResponse = serde_json::from_str(json).expect("Failed to deserialize");
        match task {
            AsyncTaskResponse::Failed(failed) => {
                assert_eq!(failed.error.code, AsyncTaskErrorCode::ServiceOverload);
                assert_eq!(failed.error.message, "Notion is overloaded.");
            }
            _ => panic!("Expected Failed variant"),
        }
    }

    #[test]
    fn deserialize_async_task_error_code_unknown() {
        let json = r#""some_future_error_code""#;
        let code: AsyncTaskErrorCode = serde_json::from_str(json).unwrap();
        assert_eq!(
            code,
            AsyncTaskErrorCode::Unknown("some_future_error_code".to_string())
        );
    }

    #[test]
    fn serialize_async_task_queued_roundtrip() {
        let task = AsyncTaskResponse::Queued(AsyncTaskProgress {
            object: "async_task".to_string(),
            id: "task-id-123".to_string(),
            status_url: "https://api.notion.com/v1/async_tasks/task-id-123".to_string(),
            created_time: "2026-01-01T00:00:00.000Z".to_string(),
            operation: AsyncTaskOperation {
                surface: AsyncTaskOperationSurface::Rest,
                name: "create_page".to_string(),
            },
            poll_after_seconds: Some(1),
        });

        let json = serde_json::to_value(&task).expect("Failed to serialize");
        assert_eq!(json["status"], "queued");
        assert_eq!(json["operation"]["surface"], "rest");

        let roundtrip: AsyncTaskResponse =
            serde_json::from_value(json).expect("Failed to deserialize");
        assert_eq!(roundtrip, task);
    }
}
