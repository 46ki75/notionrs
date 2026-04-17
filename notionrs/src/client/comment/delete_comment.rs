/// @see <https://developers.notion.com/reference/delete-a-comment>
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct DeleteCommentClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) comment_id: Option<String>,
}

impl DeleteCommentClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::comment::Comment, crate::error::Error> {
        let comment_id = self
            .comment_id
            .ok_or(crate::error::Error::RequestParameter(
                "`comment_id` is not set.".to_string(),
            ))?;

        let url = format!("https://api.notion.com/v1/comments/{}", comment_id);

        let request = self.reqwest_client.delete(url);

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
