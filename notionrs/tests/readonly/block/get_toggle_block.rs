mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/f918d678ef894e56baea3a5c765aed8b>
    static PAGE_ID: &str = "f918d678ef894e56baea3a5c765aed8b";

    #[tokio::test]
    async fn get_toggle_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let toggle_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Toggle { toggle } => Some(toggle),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(toggle_blocks.len() > 0);

        println!("{:?}", toggle_blocks);

        Ok(())
    }
}
