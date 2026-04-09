mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/6f1b1490722b4b35886a597516af953e>
    static PAGE_ID: &str = "6f1b1490722b4b35886a597516af953e";

    #[tokio::test]
    async fn get_paragraph_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let paragraph_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Paragraph { paragraph } => Some(paragraph),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(paragraph_blocks.len() > 0);

        println!("{:?}", paragraph_blocks);

        Ok(())
    }
}
