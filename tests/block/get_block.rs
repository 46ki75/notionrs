mod integration_tests {

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    async fn get_block() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        let request = client.get_block().block_id(block_id);

        let response = request.send().await?;

        match response.details {
            notionrs::block::BlockType::ChildPage { child_page } => {
                println!("Title: {}", child_page.title);
            }
            _ => panic!("Unexpected block type!"),
        }

        Ok(())
    }
}
