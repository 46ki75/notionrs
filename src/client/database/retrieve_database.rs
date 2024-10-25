use crate::error::{api_error::ApiError, Error};

#[derive(Debug, Default)]
pub struct RetrieveDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,
}

impl RetrieveDatabaseClient {
    pub async fn send(self) -> Result<crate::database::DatabaseResponse, Error> {
        let database_id = self.database_id.ok_or(Error::RequestParameter(
            "`database_id` has not been set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/databases/{}", database_id);

        let request = self.reqwest_client.get(url);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.text().await?;

            let error_json = serde_json::from_str::<ApiError>(&error_body)?;

            return Err(Error::Api(Box::new(error_json)));
        }

        let body = response.text().await?;

        let database = serde_json::from_str::<crate::database::DatabaseResponse>(&body)?;

        Ok(database)
    }

    pub fn database_id<T: AsRef<str>>(mut self, database_id: T) -> Self {
        self.database_id = Some(database_id.as_ref().to_string());
        self
    }
}
