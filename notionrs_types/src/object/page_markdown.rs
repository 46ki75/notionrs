use serde::{Deserialize, Serialize};

/// Response from the `GET /v1/pages/:id/markdown` and `PATCH /v1/pages/:id/markdown` endpoints.
///
/// <https://developers.notion.com/reference/get-page-markdown>
/// <https://developers.notion.com/reference/update-page-markdown>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageMarkdownResponse {
    /// The type of object, always `"page_markdown"`.
    pub object: String,

    /// The ID of the page or block.
    pub id: String,

    /// The page content rendered as enhanced Markdown.
    pub markdown: String,

    /// Whether the content was truncated due to exceeding the record count limit.
    pub truncated: bool,

    /// Block IDs that could not be loaded (appeared as `<unknown>` tags in the markdown).
    /// Pass these IDs back to the endpoint to fetch their content separately.
    pub unknown_block_ids: Vec<String>,

    pub request_id: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit_tests
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_page_markdown_response() {
        let json_data = r##"{
            "object": "page_markdown",
            "id": "abc123",
            "markdown": "# Hello World\n\nThis is a test.",
            "truncated": false,
            "unknown_block_ids": [],
            "request_id": "req-123"
        }"##;

        let response: PageMarkdownResponse =
            serde_json::from_str(json_data).expect("Failed to deserialize");

        assert_eq!(response.object, "page_markdown");
        assert_eq!(response.id, "abc123");
        assert_eq!(
            response.markdown,
            "# Hello World\n\nThis is a test."
        );
        assert!(!response.truncated);
        assert!(response.unknown_block_ids.is_empty());
        assert_eq!(response.request_id, Some("req-123".to_string()));
    }

    #[test]
    fn deserialize_page_markdown_response_truncated_with_unknown_blocks() {
        let json_data = r##"{
            "object": "page_markdown",
            "id": "abc123",
            "markdown": "# Hello",
            "truncated": true,
            "unknown_block_ids": ["block1", "block2"]
        }"##;

        let response: PageMarkdownResponse =
            serde_json::from_str(json_data).expect("Failed to deserialize");

        assert!(response.truncated);
        assert_eq!(response.unknown_block_ids.len(), 2);
        assert_eq!(response.unknown_block_ids[0], "block1");
        assert!(response.request_id.is_none());
    }

    #[test]
    fn serialize_page_markdown_response() {
        let response = PageMarkdownResponse {
            object: "page_markdown".to_string(),
            id: "abc123".to_string(),
            markdown: "# Hello".to_string(),
            truncated: false,
            unknown_block_ids: vec![],
            request_id: None,
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("Failed to parse as JSON");
        assert_eq!(value["object"], "page_markdown");
        assert_eq!(value["id"], "abc123");
    }
}
