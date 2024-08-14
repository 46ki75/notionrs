use crate::{
    error::{NotionApiError, NotionError},
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

        let res = self.reqwest_client.get(url).send().await?;

        if !res.status().is_success() {
            let api_error = res.json::<NotionApiError>().await?;
            return Err(NotionError::NotionApiError(Box::new(api_error)));
        }

        let body = res.json::<User>().await?;

        Ok(body)
    }
}
