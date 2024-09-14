mod integration_tests {

    use notionrs::client;
    use notionrs::error::NotionError;
    use notionrs::to_json::ToJson;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    async fn list_users() -> Result<(), NotionError> {
        let client = client::NotionClient::new();
        let res = client.list_users().send().await?;
        println!("{}", res.to_json());

        Ok(())
    }
}
