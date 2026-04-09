mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/670f691aebfc4f3e8d4aece601acba50>
    static PAGE_ID: &str = "670f691aebfc4f3e8d4aece601acba50";

    #[tokio::test]
    async fn get_callout_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let callout_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Callout { callout } => Some(callout),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(callout_blocks.len() > 0);

        println!("{:?}", callout_blocks);

        Ok(())
    }
}
