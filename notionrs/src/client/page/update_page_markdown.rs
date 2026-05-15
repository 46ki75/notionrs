use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/update-page-markdown>
///
/// This endpoint supports four operation types:
/// - `insert_content`: Insert new content into the page
/// - `replace_content_range`: Replace a range of content in the page
/// - `update_content`: Update specific content using search-and-replace operations (added in v5.13.0)
/// - `replace_content`: Replace the entire page content with new markdown (added in v5.13.0)
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct UpdatePageMarkdownClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the page to update.
    pub(crate) page_id: Option<String>,

    #[setter(skip)]
    pub(crate) body: Option<UpdatePageMarkdownBody>,
}

/// The request body for `PATCH /v1/pages/:id/markdown`.
///
/// This is a discriminated union on `type`.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UpdatePageMarkdownBody {
    /// Insert new content into the page.
    InsertContent {
        /// The insert_content payload.
        insert_content: InsertContentPayload,
    },

    /// Replace a range of content in the page.
    ReplaceContentRange {
        /// The replace_content_range payload.
        replace_content_range: ReplaceContentRangePayload,
    },

    /// Update specific content using search-and-replace operations.
    UpdateContent {
        /// The update_content payload.
        update_content: UpdateContentPayload,
    },

    /// Replace the entire page content with new markdown.
    ReplaceContent {
        /// The replace_content payload.
        replace_content: ReplaceContentPayload,
    },
}

/// Payload for the `insert_content` operation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertContentPayload {
    /// The enhanced markdown content to insert into the page.
    pub content: String,

    /// Selection of existing content to insert after, using the ellipsis format
    /// ("start text...end text"). Omit to append at the end of the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Payload for the `replace_content_range` operation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplaceContentRangePayload {
    /// The new enhanced markdown content to replace the matched range.
    pub content: String,

    /// Selection of existing content to replace, using the ellipsis format
    /// ("start text...end text").
    pub content_range: String,

    /// Set to true to allow the operation to delete child pages or databases.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deleting_content: Option<bool>,
}

/// Payload for the `update_content` operation (new in v5.13.0).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateContentPayload {
    /// An array of search-and-replace operations, each with old_str (content to find)
    /// and new_str (replacement content).
    pub content_updates: Vec<ContentUpdate>,

    /// Set to true to allow the operation to delete child pages or databases.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deleting_content: Option<bool>,
}

/// A single search-and-replace operation within `update_content`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentUpdate {
    /// The existing content string to find and replace. Must exactly match the page content.
    pub old_str: String,

    /// The new content string to replace old_str with.
    pub new_str: String,

    /// If true, replaces all occurrences of old_str. If false (default),
    /// the operation fails if there are multiple matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_all_matches: Option<bool>,
}

/// Payload for the `replace_content` operation (new in v5.13.0).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplaceContentPayload {
    /// The new enhanced markdown content to replace the entire page content.
    pub new_str: String,

    /// Set to true to allow the operation to delete child pages or databases.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deleting_content: Option<bool>,
}

impl UpdatePageMarkdownClient {
    /// Set the body to an `insert_content` operation.
    ///
    /// Inserts new content at the end of the page.
    pub fn insert_content(mut self, content: impl AsRef<str>) -> Self {
        self.body = Some(UpdatePageMarkdownBody::InsertContent {
            insert_content: InsertContentPayload {
                content: content.as_ref().to_string(),
                after: None,
            },
        });
        self
    }

    /// Set the body to an `insert_content` operation that inserts after the given selection.
    ///
    /// The `after` parameter uses the ellipsis format ("start text...end text").
    pub fn insert_content_after(
        mut self,
        content: impl AsRef<str>,
        after: impl AsRef<str>,
    ) -> Self {
        self.body = Some(UpdatePageMarkdownBody::InsertContent {
            insert_content: InsertContentPayload {
                content: content.as_ref().to_string(),
                after: Some(after.as_ref().to_string()),
            },
        });
        self
    }

    /// Set the body to a `replace_content_range` operation.
    ///
    /// The `content_range` parameter uses the ellipsis format ("start text...end text").
    pub fn replace_content_range(
        mut self,
        content: impl AsRef<str>,
        content_range: impl AsRef<str>,
    ) -> Self {
        self.body = Some(UpdatePageMarkdownBody::ReplaceContentRange {
            replace_content_range: ReplaceContentRangePayload {
                content: content.as_ref().to_string(),
                content_range: content_range.as_ref().to_string(),
                allow_deleting_content: None,
            },
        });
        self
    }

    /// Set the body to a `replace_content_range` operation, allowing deletion of child pages or databases.
    pub fn replace_content_range_allow_deleting(
        mut self,
        content: impl AsRef<str>,
        content_range: impl AsRef<str>,
        allow_deleting_content: bool,
    ) -> Self {
        self.body = Some(UpdatePageMarkdownBody::ReplaceContentRange {
            replace_content_range: ReplaceContentRangePayload {
                content: content.as_ref().to_string(),
                content_range: content_range.as_ref().to_string(),
                allow_deleting_content: Some(allow_deleting_content),
            },
        });
        self
    }

    /// Set the body to an `update_content` operation (search-and-replace).
    pub fn update_content(mut self, content_updates: Vec<ContentUpdate>) -> Self {
        self.body = Some(UpdatePageMarkdownBody::UpdateContent {
            update_content: UpdateContentPayload {
                content_updates,
                allow_deleting_content: None,
            },
        });
        self
    }

    /// Set the body to an `update_content` operation (search-and-replace),
    /// allowing deletion of child pages or databases.
    pub fn update_content_allow_deleting(
        mut self,
        content_updates: Vec<ContentUpdate>,
        allow_deleting_content: bool,
    ) -> Self {
        self.body = Some(UpdatePageMarkdownBody::UpdateContent {
            update_content: UpdateContentPayload {
                content_updates,
                allow_deleting_content: Some(allow_deleting_content),
            },
        });
        self
    }

    /// Set the body to a `replace_content` operation (replace entire page content).
    pub fn replace_content(mut self, new_str: impl AsRef<str>) -> Self {
        self.body = Some(UpdatePageMarkdownBody::ReplaceContent {
            replace_content: ReplaceContentPayload {
                new_str: new_str.as_ref().to_string(),
                allow_deleting_content: None,
            },
        });
        self
    }

    /// Set the body to a `replace_content` operation (replace entire page content),
    /// allowing deletion of child pages or databases.
    pub fn replace_content_allow_deleting(
        mut self,
        new_str: impl AsRef<str>,
        allow_deleting_content: bool,
    ) -> Self {
        self.body = Some(UpdatePageMarkdownBody::ReplaceContent {
            replace_content: ReplaceContentPayload {
                new_str: new_str.as_ref().to_string(),
                allow_deleting_content: Some(allow_deleting_content),
            },
        });
        self
    }

    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page_markdown::PageMarkdownResponse, crate::error::Error>
    {
        let page_id = self.page_id.ok_or(crate::error::Error::RequestParameter(
            "`page_id` is not set.".to_string(),
        ))?;

        let body = self.body.ok_or(crate::error::Error::RequestParameter(
            "No operation body is set. Use one of `insert_content`, `replace_content_range`, `update_content`, or `replace_content`.".to_string(),
        ))?;

        let request_body = serde_json::to_string(&body)?;

        let url = format!("https://api.notion.com/v1/pages/{}/markdown", page_id);

        let request = self
            .reqwest_client
            .patch(url)
            .header("Content-Type", "application/json")
            .body(request_body);

        let response = request
            .send()
            .await
            .map_err(|e| crate::error::Error::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(crate::error::Error::try_from_response_async(response).await);
        }

        let body = response
            .bytes()
            .await
            .map_err(|e| crate::error::Error::BodyParse(e.to_string()))?;

        let page_markdown = serde_json::from_slice::<
            notionrs_types::object::page_markdown::PageMarkdownResponse,
        >(&body)?;

        Ok(page_markdown)
    }
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
    fn serialize_insert_content() {
        let body = UpdatePageMarkdownBody::InsertContent {
            insert_content: InsertContentPayload {
                content: "# New Heading\n\nSome text.".to_string(),
                after: None,
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(json["type"], "insert_content");
        assert_eq!(json["insert_content"]["content"], "# New Heading\n\nSome text.");
        assert!(json["insert_content"].get("after").is_none());
    }

    #[test]
    fn serialize_insert_content_with_after() {
        let body = UpdatePageMarkdownBody::InsertContent {
            insert_content: InsertContentPayload {
                content: "New content".to_string(),
                after: Some("start...end".to_string()),
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(json["type"], "insert_content");
        assert_eq!(json["insert_content"]["after"], "start...end");
    }

    #[test]
    fn serialize_replace_content_range() {
        let body = UpdatePageMarkdownBody::ReplaceContentRange {
            replace_content_range: ReplaceContentRangePayload {
                content: "replacement text".to_string(),
                content_range: "old start...old end".to_string(),
                allow_deleting_content: None,
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(json["type"], "replace_content_range");
        assert_eq!(json["replace_content_range"]["content"], "replacement text");
        assert_eq!(
            json["replace_content_range"]["content_range"],
            "old start...old end"
        );
        assert!(json["replace_content_range"]
            .get("allow_deleting_content")
            .is_none());
    }

    #[test]
    fn serialize_replace_content_range_with_allow_deleting() {
        let body = UpdatePageMarkdownBody::ReplaceContentRange {
            replace_content_range: ReplaceContentRangePayload {
                content: "new".to_string(),
                content_range: "old".to_string(),
                allow_deleting_content: Some(true),
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(
            json["replace_content_range"]["allow_deleting_content"],
            true
        );
    }

    #[test]
    fn serialize_update_content() {
        let body = UpdatePageMarkdownBody::UpdateContent {
            update_content: UpdateContentPayload {
                content_updates: vec![
                    ContentUpdate {
                        old_str: "old text".to_string(),
                        new_str: "new text".to_string(),
                        replace_all_matches: None,
                    },
                    ContentUpdate {
                        old_str: "heading".to_string(),
                        new_str: "updated heading".to_string(),
                        replace_all_matches: Some(true),
                    },
                ],
                allow_deleting_content: None,
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(json["type"], "update_content");
        let updates = json["update_content"]["content_updates"].as_array().unwrap();
        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0]["old_str"], "old text");
        assert_eq!(updates[0]["new_str"], "new text");
        assert!(updates[0].get("replace_all_matches").is_none());
        assert_eq!(updates[1]["old_str"], "heading");
        assert_eq!(updates[1]["new_str"], "updated heading");
        assert_eq!(updates[1]["replace_all_matches"], true);
    }

    #[test]
    fn serialize_update_content_with_allow_deleting() {
        let body = UpdatePageMarkdownBody::UpdateContent {
            update_content: UpdateContentPayload {
                content_updates: vec![ContentUpdate {
                    old_str: "old".to_string(),
                    new_str: "new".to_string(),
                    replace_all_matches: None,
                }],
                allow_deleting_content: Some(true),
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(json["update_content"]["allow_deleting_content"], true);
    }

    #[test]
    fn serialize_replace_content() {
        let body = UpdatePageMarkdownBody::ReplaceContent {
            replace_content: ReplaceContentPayload {
                new_str: "# Entire new page content".to_string(),
                allow_deleting_content: None,
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(json["type"], "replace_content");
        assert_eq!(
            json["replace_content"]["new_str"],
            "# Entire new page content"
        );
        assert!(json["replace_content"]
            .get("allow_deleting_content")
            .is_none());
    }

    #[test]
    fn serialize_replace_content_with_allow_deleting() {
        let body = UpdatePageMarkdownBody::ReplaceContent {
            replace_content: ReplaceContentPayload {
                new_str: "# New content".to_string(),
                allow_deleting_content: Some(true),
            },
        };

        let json = serde_json::to_value(&body).expect("Failed to serialize");
        assert_eq!(json["replace_content"]["allow_deleting_content"], true);
    }

    #[test]
    fn deserialize_insert_content() {
        let json = r#"
        {
            "type": "insert_content",
            "insert_content": {
                "content": "hello"
            }
        }
        "#;

        let body: UpdatePageMarkdownBody =
            serde_json::from_str(json).expect("Failed to deserialize");
        match body {
            UpdatePageMarkdownBody::InsertContent { insert_content } => {
                assert_eq!(insert_content.content, "hello");
                assert!(insert_content.after.is_none());
            }
            _ => panic!("Expected InsertContent variant"),
        }
    }

    #[test]
    fn deserialize_replace_content_range() {
        let json = r#"
        {
            "type": "replace_content_range",
            "replace_content_range": {
                "content": "new",
                "content_range": "old start...old end"
            }
        }
        "#;

        let body: UpdatePageMarkdownBody =
            serde_json::from_str(json).expect("Failed to deserialize");
        match body {
            UpdatePageMarkdownBody::ReplaceContentRange {
                replace_content_range,
            } => {
                assert_eq!(replace_content_range.content, "new");
                assert_eq!(replace_content_range.content_range, "old start...old end");
                assert!(replace_content_range.allow_deleting_content.is_none());
            }
            _ => panic!("Expected ReplaceContentRange variant"),
        }
    }

    #[test]
    fn deserialize_update_content() {
        let json = r#"
        {
            "type": "update_content",
            "update_content": {
                "content_updates": [
                    {
                        "old_str": "old text",
                        "new_str": "new text"
                    },
                    {
                        "old_str": "heading",
                        "new_str": "updated heading",
                        "replace_all_matches": true
                    }
                ],
                "allow_deleting_content": false
            }
        }
        "#;

        let body: UpdatePageMarkdownBody =
            serde_json::from_str(json).expect("Failed to deserialize");
        match body {
            UpdatePageMarkdownBody::UpdateContent { update_content } => {
                assert_eq!(update_content.content_updates.len(), 2);
                assert_eq!(update_content.content_updates[0].old_str, "old text");
                assert_eq!(update_content.content_updates[0].new_str, "new text");
                assert!(update_content.content_updates[0].replace_all_matches.is_none());
                assert_eq!(update_content.content_updates[1].old_str, "heading");
                assert_eq!(
                    update_content.content_updates[1].replace_all_matches,
                    Some(true)
                );
                assert_eq!(update_content.allow_deleting_content, Some(false));
            }
            _ => panic!("Expected UpdateContent variant"),
        }
    }

    #[test]
    fn deserialize_replace_content() {
        let json = r##"
        {
            "type": "replace_content",
            "replace_content": {
                "new_str": "# Complete replacement",
                "allow_deleting_content": true
            }
        }
        "##;

        let body: UpdatePageMarkdownBody =
            serde_json::from_str(json).expect("Failed to deserialize");
        match body {
            UpdatePageMarkdownBody::ReplaceContent { replace_content } => {
                assert_eq!(replace_content.new_str, "# Complete replacement");
                assert_eq!(replace_content.allow_deleting_content, Some(true));
            }
            _ => panic!("Expected ReplaceContent variant"),
        }
    }
}
