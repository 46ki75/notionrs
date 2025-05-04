#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct RetrieveDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,
}

impl RetrieveDatabaseClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::database::DatabaseResponse, crate::error::Error> {
        let database_id = self
            .database_id
            .ok_or(crate::error::Error::RequestParameter(
                "`database_id` is not set.".to_string(),
            ))?;

        let url = format!("https://api.notion.com/v1/databases/{}", database_id);

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

        let database =
            serde_json::from_slice::<notionrs_types::object::database::DatabaseResponse>(&body)?;

        Ok(database)
    }
}
