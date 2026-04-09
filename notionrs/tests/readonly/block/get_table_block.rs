mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/0004370e00c94e07a1ff60c9c6f905ff>
    static PAGE_ID: &str = "0004370e00c94e07a1ff60c9c6f905ff";

    #[tokio::test]
    async fn get_table_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly"))
            .expect("Failed to load .env.readonly file");

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let table_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Table { table } => Some(table),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(table_blocks.len() > 0);

        println!("{:?}", table_blocks);

        Ok(())
    }
}
