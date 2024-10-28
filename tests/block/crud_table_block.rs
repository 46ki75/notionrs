mod integration_tests {

    #[tokio::test]
    async fn crud_table_block() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let block_id = std::env::var("NOTION_IT_CRUD_PAGE_ID").unwrap();

        let client = notionrs::client::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = notionrs::RichText::from("rich text");

        let block = notionrs::block::Block::Table {
            table: notionrs::block::TableBlock::new()
                .table_width(2)
                .has_column_header(true)
                .has_row_header(true)
                .children(vec![
                    notionrs::block::Block::TableRow {
                        table_row: notionrs::block::TableRowBlock::new()
                            .cells(vec![vec![rich_text.clone()], vec![rich_text.clone()]]),
                    },
                    notionrs::block::Block::TableRow {
                        table_row: notionrs::block::TableRowBlock::new()
                            .cells(vec![vec![rich_text.clone()], vec![rich_text.clone()]]),
                    },
                    notionrs::block::Block::TableRow {
                        table_row: notionrs::block::TableRowBlock::new()
                            .cells(vec![vec![rich_text.clone()], vec![rich_text.clone()]]),
                    },
                ]),
        };

        println!("{:?}", block);

        let request = client
            .append_block_children()
            .block_id(block_id.clone())
            .children(vec![block]);

        let response = request.send().await?;

        // # --------------------------------------------------------------------------------
        //
        // get_block
        //
        // # --------------------------------------------------------------------------------

        let request = client
            .get_block()
            .block_id(response.results.first().unwrap().id.clone());

        let response = request.send().await?;

        // # --------------------------------------------------------------------------------
        //
        // delete_block
        //
        // # --------------------------------------------------------------------------------

        let request = client.delete_block().block_id(response.id.clone());

        let response = request.send().await?;

        println!("{:?}", response);

        Ok(())
    }
}
