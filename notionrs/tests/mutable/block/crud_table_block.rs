mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b2680cfbea9c758b76a8cbe>
    static PAGE_ID: &str = "33da03d79b2680cfbea9c758b76a8cbe";

    #[tokio::test]
    async fn crud_table_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = RichText::from("rich text");

        let block = Block::Table {
            table: TableBlock::default()
                .table_width(2)
                .has_column_header(true)
                .has_row_header(true)
                .children(vec![
                    Block::TableRow {
                        table_row: TableRowBlock::default()
                            .cells(vec![vec![rich_text.clone()], vec![rich_text.clone()]]),
                    },
                    Block::TableRow {
                        table_row: TableRowBlock::default()
                            .cells(vec![vec![rich_text.clone()], vec![rich_text.clone()]]),
                    },
                    Block::TableRow {
                        table_row: TableRowBlock::default()
                            .cells(vec![vec![rich_text.clone()], vec![rich_text.clone()]]),
                    },
                ]),
        };

        println!("{:?}", block);

        let request = client
            .append_block_children()
            .block_id(PAGE_ID)
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
