mod integration_tests {

    /// <https://www.notion.so/33da03d79b2680389ed7d9d82efcb3c3>
    static PAGE_ID: &str = "33da03d79b2680389ed7d9d82efcb3c3";

    #[tokio::test]
    async fn retrieve_comments() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.retrieve_comments().block_id(PAGE_ID);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        assert!(response.results.len() > 0);

        Ok(())
    }
}
