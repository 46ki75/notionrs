use crate::{
    error::{api_error::ApiError, Error},
    user::User,
};

#[derive(Debug)]
pub struct GetUserClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) user_id: Option<String>,
}

impl GetUserClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<User, Error> {
        match self.user_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/users/{}", id);

                let request = self.reqwest_client.get(url);

                let response = request.send().await?;

                if !response.status().is_success() {
                    let error_body = response.text().await?;

                    let error_json = serde_json::from_str::<ApiError>(&error_body)?;

                    return Err(Error::Api(Box::new(error_json)));
                }

                let body = response.text().await?;

                let user = serde_json::from_str::<User>(&body)?;

                Ok(user)
            }
            None => Err(Error::RequestParameter("user_id is empty".to_string())),
        }
    }

    /// Sets the user ID.
    pub fn user_id<T: AsRef<str>>(mut self, user_id: T) -> Self {
        self.user_id = Some(user_id.as_ref().to_string());
        self
    }
}
