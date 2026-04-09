mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/fd9db645268e48de8d10ef5982a64e60>
    static PAGE_ID: &str = "fd9db645268e48de8d10ef5982a64e60";

    #[tokio::test]
    async fn get_column_list_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let column_list_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::ColumnList { column_list } => Some(column_list),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(column_list_blocks.len() > 0);

        println!("{:?}", column_list_blocks);

        Ok(())
    }
}
