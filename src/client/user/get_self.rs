use crate::{
    error::{NotionApiError, NotionError},
    user::User,
};

pub struct GetSelfClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,
}

impl GetSelfClient {
    /// https://developers.notion.com/reference/get-self
    ///
    /// Retrieves information about the user (bot) associated with the currently used token.
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
    ///     let result = notion.get_self().send().await?;
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

// # --------------------------------------------------------------------------------
//
// Integration Test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::client;
    use crate::to_json::ToJson;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    #[ignore]
    async fn integration_test_get_self() {
        let method_name = "get_self";
        println!("\n\x1b[30;47m # {} # \x1b[0m", method_name);

        let client = client::NotionClient::new();
        let res = client.get_self().send().await.unwrap();
        println!("{}", res.to_json());
    }
}
