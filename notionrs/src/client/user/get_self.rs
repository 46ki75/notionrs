#[derive(Debug)]
pub struct GetSelfClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,
}

impl GetSelfClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<crate::user::bot::Bot, crate::error::Error> {
        let url = String::from("https://api.notion.com/v1/users/me");

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

        let user = serde_json::from_slice::<crate::user::bot::Bot>(&body)?;

        Ok(user)
    }
}
