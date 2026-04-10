mod integration_tests {

    /// <https://www.notion.so/33ea03d79b268008b53ff73fcd33f45e?v=33ea03d79b2680e2bba8000cbc556474>
    static VIEW_ID: &str = "33ea03d79b2680e2bba8000cbc556474";

    #[tokio::test]
    async fn retrieve_view() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();

        let client = notionrs::Client::new(notion_api_key);

        let view = client.retrieve_view().view_id(VIEW_ID).send().await?;

        println!("{}", serde_json::to_string(&view)?);

        Ok(())
    }
}
