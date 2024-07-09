use crate::{
    error::{NotionApiError, NotionError},
    user::User,
};

pub struct GetUserClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) user_id: Option<String>,
}

impl GetUserClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<User, NotionError> {
        match self.user_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/users/{}", id);

                let res = self.reqwest_client.get(url).send().await?;

                if !res.status().is_success() {
                    let api_error = res.json::<NotionApiError>().await?;
                    return Err(NotionError::NotionApiError(Box::new(api_error)));
                }

                let body = res.json::<User>().await?;

                Ok(body)
            }
            None => Err(NotionError::NotionRequestParameterError(
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
