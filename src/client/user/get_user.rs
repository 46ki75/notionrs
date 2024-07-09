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
    /// https://developers.notion.com/reference/get-user
    ///
    /// Can be used to retrieve a specific user.
    ///
    /// ## Usage
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::NotionError;
    /// use notionrs::prelude::ToJson;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let result = notion
    ///         .get_user()
    ///         .user_id("c4e69ebe-3c42-4916-8ec4-285e5cb5bcb0")
    ///         .send()
    ///         .await?;
    ///     println!("{}", result.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    /// ## Sample response
    ///
    /// ```json
    /// {
    ///     "object": "user",
    ///     "id": "3571265d-852e-4aec-b529-75947e7842d6",
    ///     "name": "default",
    ///     "avatar_url": null,
    ///     "type": "bot",
    ///     "bot": {
    ///         "owner": {
    ///             "type": "workspace",
    ///             "workspace": true
    ///         },
    ///         "workspace_name": "MyWorkspace"
    ///     },
    ///     "request_id": "1739014e-262a-4592-b2c3-9b76491a5ed1"
    /// }
    /// ```
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
    /// ## Usage
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::NotionError;
    /// use notionrs::prelude::ToJson;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let result = notion
    ///         .get_user()
    ///         .user_id("c4e69ebe-3c42-4916-8ec4-285e5cb5bcb0")
    ///         .send()
    ///         .await?;
    ///     println!("{}", result.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn user_id<T: AsRef<str>>(mut self, user_id: T) -> Self {
        self.user_id = Some(user_id.as_ref().to_string());
        self
    }
}

// # --------------------------------------------------------------------------------
//
// Integration Test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::client;
    use crate::to_json::ToJson;

    use dotenv::dotenv;
    use std::env;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    ///
    /// To conduct integration testing, write the following in the .env file.
    /// ```ini
    /// NOTION_USER_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    /// ```
    #[tokio::test]
    #[ignore]
    async fn integration_test_get_user() {
        let method_name = "get_user";
        println!("\n\x1b[30;47m # {} # \x1b[0m", method_name);

        dotenv().ok();
        let user_id = env::var("NOTION_USER_ID").unwrap_or_else(|_| String::new());

        let client = client::NotionClient::new();
        let res = client.get_user().user_id(user_id).send().await.unwrap();
        println!("{}", res.to_json());
    }
}
