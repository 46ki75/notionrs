mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/adeda3bccc6044628aa613bd5026dd59>
    static PAGE_ID: &str = "adeda3bccc6044628aa613bd5026dd59";

    #[tokio::test]
    async fn get_heading_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let heading_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Heading1 { heading_1 } => Some(heading_1),
                Block::Heading2 { heading_2 } => Some(heading_2),
                Block::Heading3 { heading_3 } => Some(heading_3),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(heading_blocks.len() > 0);

        println!("{:?}", heading_blocks);

        Ok(())
    }
}
