mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/5925771ca5ea4576b7180980b280e2f1>
    static PAGE_ID: &str = "5925771ca5ea4576b7180980b280e2f1";

    #[tokio::test]
    async fn get_to_do_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly"))
            .expect("Failed to load .env.readonly file");

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let to_do_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::ToDo { to_do } => Some(to_do),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(to_do_blocks.len() > 0);

        println!("{:?}", to_do_blocks);

        Ok(())
    }
}
