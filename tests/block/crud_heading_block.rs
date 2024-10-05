mod integration_tests {

    #[tokio::test]
    async fn crud_heading_block() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let request = client
            .append_block_children()
            .block_id(block_id.clone())
            .children(vec![notionrs::block::Block::new_heading()
                .rich_text(vec![notionrs::rich_text!("my heading")])
                .level(2)
                .build()]);

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
        // update_block
        //
        // # --------------------------------------------------------------------------------

        let block = match response.details {
            notionrs::block::Block::Heading2 { heading_2 } => heading_2,
            e => panic!("{:?}", e),
        };

        let builded_block = block
            .rich_text(vec![notionrs::rich_text!("my heading?")])
            .level(2)
            .build();

        let request = client
            .update_block()
            .block_id(response.id.clone())
            .block(builded_block);

        let response = request.send().await?;

        println!("{:?}", response);

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
