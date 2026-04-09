mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/3014de33189946859bd488fa04ec15d0>
    static PAGE_ID: &str = "3014de33189946859bd488fa04ec15d0";

    #[tokio::test]
    async fn get_tab_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let tab_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Tab { tab } => Some(tab),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(tab_blocks.len() > 0);

        println!("{:?}", tab_blocks);

        Ok(())
    }
}
