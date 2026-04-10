/// <https://developers.notion.com/reference/get-page-markdown>
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct GetPageMarkdownClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the page (or block) to retrieve as markdown.
    /// Non-navigable block IDs from truncated responses can be passed here
    /// to fetch their subtrees.
    pub(crate) page_id: Option<String>,

    /// Whether to include meeting note transcripts. Defaults to false.
    /// When true, full transcripts are included; when false, a placeholder
    /// with the meeting note URL is shown instead.
    pub(crate) include_transcript: Option<bool>,
}

impl GetPageMarkdownClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page_markdown::PageMarkdownResponse, crate::error::Error>
    {
        let page_id = self.page_id.ok_or(crate::error::Error::RequestParameter(
            "`page_id` is not set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/pages/{}/markdown", page_id);

        let request = self.reqwest_client.get(url);
        let request = if let Some(include_transcript) = self.include_transcript {
            request.query(&[("include_transcript", include_transcript)])
        } else {
            request
        };

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
