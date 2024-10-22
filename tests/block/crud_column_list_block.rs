mod integration_tests {

    #[tokio::test]
    async fn crud_column_list_block() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = notionrs::RichText::from("child");

        let grandchildren = notionrs::block::Block::Paragraph {
            paragraph: notionrs::block::ParagraphBlock::new().rich_text(vec![rich_text.clone()]),
        };

        let child = notionrs::block::Block::Column {
            column: notionrs::block::ColumnBlock::new().children(vec![grandchildren]),
        };

        let block = notionrs::block::Block::ColumnList {
            column_list: notionrs::block::ColumnListBlock::new()
                .children(vec![child.clone(), child.clone()]),
        };

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