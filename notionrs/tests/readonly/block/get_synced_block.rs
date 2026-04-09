mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/3cacf16ec0b84ec3b51e787bae1d562f>
    static PAGE_ID: &str = "3cacf16ec0b84ec3b51e787bae1d562f";

    #[tokio::test]
    async fn get_synced_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let synced_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::SyncedBlock { synced_block } => Some(synced_block),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(synced_blocks.len() > 0);

        println!("{:?}", synced_blocks);

        Ok(())
    }
}
