mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/33da03d79b2680cbbf53ece489bdb1b8>
    static PAGE_ID: &str = "33da03d79b2680cbbf53ece489bdb1b8";

    #[tokio::test]
    async fn get_bulleted_list_item_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let bulleted_list_item_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::BulletedListItem { bulleted_list_item } => Some(bulleted_list_item),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(bulleted_list_item_blocks.len() > 0);

        println!("{:?}", bulleted_list_item_blocks);

        Ok(())
    }
}
