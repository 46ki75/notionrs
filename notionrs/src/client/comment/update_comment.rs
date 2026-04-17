use serde::Serialize;

/// @see <https://developers.notion.com/reference/update-a-comment>
///
/// Note: `rich_text` and `markdown` are mutually exclusive. Please set only one of them.
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct UpdateCommentClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) comment_id: Option<String>,

    /// Mutually exclusive with `markdown`.
    /// Please set only one of either `rich_text` or `markdown`.
    pub(crate) rich_text: Option<Vec<notionrs_types::object::rich_text::RichText>>,

    /// The updated content of the comment as a Markdown string.
    /// Supports inline formatting (bold, italic, strikethrough, code, links),
    /// inline equations ($expression$), and mentions.
    /// Mutually exclusive with `rich_text`.
    /// Please set only one of either `rich_text` or `markdown`.
    pub(crate) markdown: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateCommentRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    rich_text: Option<Vec<notionrs_types::object::rich_text::RichText>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    markdown: Option<String>,
}

impl UpdateCommentClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::comment::Comment, crate::error::Error> {
        let comment_id = self
            .comment_id
            .ok_or(crate::error::Error::RequestParameter(
                "`comment_id` is not set.".to_string(),
            ))?;

        if self.rich_text.is_none() && self.markdown.is_none() {
            return Err(crate::error::Error::RequestParameter(
                "either rich_text or markdown must be provided".to_string(),
            ));
        }

        if self.rich_text.is_some() && self.markdown.is_some() {
            return Err(crate::error::Error::RequestParameter(
                "`rich_text` and `markdown` are mutually exclusive. Please set only one of them."
                    .to_string(),
            ));
        }

        let url = format!("https://api.notion.com/v1/comments/{}", comment_id);

        let body = UpdateCommentRequestBody {
            rich_text: self.rich_text,
            markdown: self.markdown,
        };

        let body_string = serde_json::to_string(&body)?;

        let request = self
            .reqwest_client
            .patch(url)
            .header("Content-Type", "application/json")
            .body(body_string);

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

        let comment = serde_json::from_slice::<notionrs_types::object::comment::Comment>(&body)?;

        Ok(comment)
    }
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
    fn serialize_request_body_with_rich_text() {
        let body = UpdateCommentRequestBody {
            rich_text: Some(vec![notionrs_types::object::rich_text::RichText::from(
                "Updated comment text",
            )]),
            markdown: None,
        };

        let json = serde_json::to_value(&body).unwrap();

        assert!(json.get("rich_text").is_some());
        assert!(json.get("markdown").is_none());
    }

    #[test]
    fn serialize_request_body_with_markdown() {
        let body = UpdateCommentRequestBody {
            rich_text: None,
            markdown: Some("**bold** and _italic_".to_string()),
        };

        let json = serde_json::to_value(&body).unwrap();

        assert!(json.get("markdown").is_some());
        assert_eq!(
            json.get("markdown").unwrap().as_str().unwrap(),
            "**bold** and _italic_"
        );
        assert!(json.get("rich_text").is_none());
    }
}
