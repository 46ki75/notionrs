mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/783f1986a43b48e6a305891302ce163a>
    static PAGE_ID: &str = "783f1986a43b48e6a305891302ce163a";

    #[tokio::test]
    async fn get_equation_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly"))
            .expect("Failed to load .env.readonly file");

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let equation_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Equation { equation } => Some(equation),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(equation_blocks.len() > 0);

        println!("{:?}", equation_blocks);

        Ok(())
    }
}
