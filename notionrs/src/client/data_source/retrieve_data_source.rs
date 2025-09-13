#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct RetrieveDataSourceClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) data_source_id: Option<String>,
}

impl RetrieveDataSourceClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::data_source::DataSourceResponse, crate::error::Error> {
        let data_source_id = self
            .data_source_id
            .ok_or(crate::error::Error::RequestParameter(
                "`data_source_id` is not set.".to_string(),
            ))?;

        let url = format!("https://api.notion.com/v1/data_sources/{}", data_source_id);

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

        let database = serde_json::from_slice::<
            notionrs_types::object::data_source::DataSourceResponse,
        >(&body)?;

        Ok(database)
    }
}
