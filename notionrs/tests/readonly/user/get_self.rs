mod integration_tests {

    use notionrs::Error;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    async fn get_self() -> Result<(), Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let res = client.get_self().send().await?;
        println!("{}", serde_json::to_string(&res)?);

        Ok(())
    }
}
