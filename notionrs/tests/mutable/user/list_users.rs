mod integration_tests {

    use futures::TryStreamExt;
    use notionrs::{Error, r#trait::PaginateExt};

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    async fn list_users() -> Result<(), Error> {
        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let res = client.list_users().send().await?;
        println!("{}", serde_json::to_string(&res)?);

        Ok(())
    }

    #[tokio::test]
    async fn list_users_all() -> Result<(), Error> {
        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let res: Vec<notionrs_types::prelude::User> = client
            .list_users()
            .into_stream()
            .try_collect()
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&res)?);

        Ok(())
    }
}
