mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/33da03d79b26807fb622dcc2fd5bac11>
    static PAGE_ID: &str = "33da03d79b26807fb622dcc2fd5bac11";

    #[tokio::test]
    async fn get_breadcrumb_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let breadcrumb_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Breadcrumb { breadcrumb } => Some(breadcrumb),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(breadcrumb_blocks.len() > 0);

        println!("{:?}", breadcrumb_blocks);

        Ok(())
    }
}
