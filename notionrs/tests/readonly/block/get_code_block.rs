mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/012927e34dce43e2968bca013e77480a>
    static PAGE_ID: &str = "012927e34dce43e2968bca013e77480a";

    #[tokio::test]
    async fn get_code_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let code_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Code { code } => Some(code),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(code_blocks.len() > 0);

        println!("{:?}", code_blocks);

        Ok(())
    }
}
