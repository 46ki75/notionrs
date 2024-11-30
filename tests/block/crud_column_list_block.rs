mod integration_tests {

    #[tokio::test]
    async fn crud_column_list_block() -> Result<(), notionrs::error::Error> {
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

        let rich_text = notionrs::RichText::from("child");

        let grandchildren = notionrs::block::Block::Paragraph {
            paragraph: notionrs::block::ParagraphBlock::default()
                .rich_text(vec![rich_text.clone()]),
        };

        let child = notionrs::block::Block::Column {
            column: notionrs::block::ColumnBlock::default().children(vec![grandchildren]),
        };

        let block = notionrs::block::Block::ColumnList {
            column_list: notionrs::block::ColumnListBlock::default()
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
