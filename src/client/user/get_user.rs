use crate::user::User;

#[derive(Debug)]
pub struct GetUserClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) user_id: Option<String>,
}

impl GetUserClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<User, crate::error::Error> {
        match self.user_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/users/{}", id);

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

                let user = serde_json::from_slice::<User>(&body)?;

                Ok(user)
            }
            None => Err(crate::error::Error::RequestParameter(
                "user_id is empty".to_string(),
            )),
        }
    }

    /// Sets the user ID.
    pub fn user_id<T: AsRef<str>>(mut self, user_id: T) -> Self {
        self.user_id = Some(user_id.as_ref().to_string());
        self
    }
}
