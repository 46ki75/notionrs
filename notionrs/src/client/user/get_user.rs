use notionrs_types::object::user::User;

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct GetUserClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the user to retrieve.
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
                "`user_id` is not set.".to_string(),
            )),
        }
    }
}
