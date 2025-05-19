use serde::Serialize;

#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct RetrieveCommentsClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) block_id: Option<String>,

    pub(crate) page_size: Option<u8>,

    pub(crate) start_cursor: Option<String>,
}

#[async_trait::async_trait]
impl crate::r#trait::Paginate<notionrs_types::object::comment::Comment> for RetrieveCommentsClient {
    fn paginate_start_cursor(self, start_cursor: Option<String>) -> Self {
        match start_cursor {
            Some(c) => self.start_cursor(c),
            None => self,
        }
    }

    async fn paginate_send(
        self,
    ) -> Result<
        notionrs_types::object::response::ListResponse<notionrs_types::object::comment::Comment>,
        crate::error::Error,
    > {
        Ok(self.send().await?)
    }
}

#[derive(Serialize)]
struct RetrieveCommentsQueryParams {
    block_id: String,

    start_cursor: Option<String>,

    page_size: Option<u8>,
}

impl RetrieveCommentsClient {
    pub async fn send(
        self,
    ) -> Result<
        notionrs_types::object::response::ListResponse<notionrs_types::object::comment::Comment>,
        crate::error::Error,
    > {
        match self.block_id {
            None => Err(crate::error::Error::RequestParameter(
                "block_id is not set.".to_string(),
            )),
            Some(block_id) => {
                let url = "https://api.notion.com/v1/comments";

                let query = RetrieveCommentsQueryParams {
                    block_id,
                    start_cursor: self.start_cursor,
                    page_size: self.page_size,
                };

                let request = self.reqwest_client.get(url).query(&query);

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

                let comments = serde_json::from_slice::<
                    notionrs_types::object::response::ListResponse<
                        notionrs_types::object::comment::Comment,
                    >,
                >(&body)?;

                Ok(comments)
            }
        }
    }
}
