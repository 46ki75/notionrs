use crate::{
    error::{api_error::NotionApiError, NotionError},
    user::User,
};

#[derive(Debug)]
pub struct GetSelfClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,
}

impl GetSelfClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<User, NotionError> {
        let url = String::from("https://api.notion.com/v1/users/me");

        let request = self.reqwest_client.get(url);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.text().await?;

            let error_json = serde_json::from_str::<NotionApiError>(&error_body)?;

            return Err(NotionError::NotionApiError(Box::new(error_json)));
        }

        let body = response.text().await?;

        let user = serde_json::from_str::<User>(&body)?;

        Ok(user)
    }
}
