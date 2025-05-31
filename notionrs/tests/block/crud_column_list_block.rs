mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn crud_column_list_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let block_id = std::env::var("NOTION_IT_CRUD_PAGE_ID").unwrap();

        let client = notionrs::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = RichText::from("child");

        let grandchildren = Block::Paragraph {
            paragraph: ParagraphBlock::default().rich_text(vec![rich_text.clone()]),
        };

        let child = Block::Column {
            column: ColumnBlock::default().children(vec![grandchildren]),
        };

        let block = Block::ColumnList {
            column_list: ColumnListBlock::default().children(vec![
                child.clone(),
                child.clone(),
                child.clone(),
            ]),
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
