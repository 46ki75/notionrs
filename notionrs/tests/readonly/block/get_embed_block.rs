mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/a09171f09f3b4afbb3e5f3afafaf9b66>
    static PAGE_ID: &str = "a09171f09f3b4afbb3e5f3afafaf9b66";

    #[tokio::test]
    async fn get_embed_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let embed_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Embed { embed } => Some(embed),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(embed_blocks.len() > 0);

        println!("{:?}", embed_blocks);

        Ok(())
    }
}
