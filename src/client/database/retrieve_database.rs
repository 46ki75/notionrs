#[derive(Debug, Default)]
pub struct RetrieveDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,
}

impl RetrieveDatabaseClient {
    pub async fn send(self) -> Result<crate::database::DatabaseResponse, crate::error::Error> {
        let database_id = self
            .database_id
            .ok_or(crate::error::Error::RequestParameter(
                "`database_id` has not been set.".to_string(),
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

        let database = serde_json::from_slice::<crate::database::DatabaseResponse>(&body)?;

        Ok(database)
    }

    pub fn database_id<T: AsRef<str>>(mut self, database_id: T) -> Self {
        self.database_id = Some(database_id.as_ref().to_string());
        self
    }
}
