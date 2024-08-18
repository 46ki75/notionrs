mod integration_tests {
    use notionrs::to_json::ToJson;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    async fn append_block_children() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        let block = notionrs::block::BlockType::bookmark("https://example.com")
            .caption(vec![])
            .build();

        let blocks = vec![block];

        let request = client
            .append_block_children()
            .block_id(block_id)
            .children(blocks);

        let response = request.send().await?;

        println!("{:?}", response.to_json());

        Ok(())
    }
}
