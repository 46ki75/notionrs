#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct GetPageClient {
    pub(crate) reqwest_client: reqwest::Client,

    /// Specify the ID of the page.
    /// The ID is also included in the Notion page URL.
    pub(crate) page_id: Option<String>,
}

impl GetPageClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_schema::object::page::PageResponse, crate::error::Error> {
        match self.page_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/pages/{}", id);

                let request = self.reqwest_client.get(url);

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

                let page =
                    serde_json::from_slice::<notionrs_schema::object::page::PageResponse>(&body)?;

                Ok(page)
            }
            None => Err(crate::error::Error::RequestParameter(
                "`user_id` is not set.".to_string(),
            )),
        }
    }
}
