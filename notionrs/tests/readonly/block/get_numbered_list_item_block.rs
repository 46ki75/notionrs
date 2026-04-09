mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/30d6146fbe204ee190174f15806bc843>
    static PAGE_ID: &str = "30d6146fbe204ee190174f15806bc843";

    #[tokio::test]
    async fn get_numbered_list_item_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let numbered_list_item_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::NumberedListItem { numbered_list_item } => Some(numbered_list_item),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(numbered_list_item_blocks.len() > 0);

        println!("{:?}", numbered_list_item_blocks);

        Ok(())
    }
}
