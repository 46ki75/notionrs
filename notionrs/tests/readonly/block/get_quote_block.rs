mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/b85facaf60bc402ebad5b1c48aa9d441>
    static PAGE_ID: &str = "b85facaf60bc402ebad5b1c48aa9d441";

    #[tokio::test]
    async fn get_quote_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly"))
            .expect("Failed to load .env.readonly file");

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let quote_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Quote { quote } => Some(quote),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(quote_blocks.len() > 0);

        println!("{:?}", quote_blocks);

        Ok(())
    }
}
