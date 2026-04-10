/// <https://developers.notion.com/reference/delete-a-view>
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct DeleteViewClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the view to delete.
    pub(crate) view_id: Option<String>,
}

impl DeleteViewClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::view::PartialViewResponse, crate::error::Error> {
        let view_id = self.view_id.ok_or(crate::error::Error::RequestParameter(
            "`view_id` is not set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/views/{}", view_id);

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

        let view =
            serde_json::from_slice::<notionrs_types::object::view::PartialViewResponse>(&body)?;

        Ok(view)
    }
}
