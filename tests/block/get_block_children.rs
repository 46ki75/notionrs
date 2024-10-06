mod integration_tests {

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    async fn get_block_children() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        let request = client.get_block_children().block_id(block_id);

        let _ = request.send().await?;

        Ok(())
    }
}
