mod integration_tests {

    #[tokio::test]
    async fn crud_bookmark_block() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let block = notionrs::block::Block::Bookmark {
            bookmark: notionrs::block::BookmarkBlock::new().url("https://example.com"),
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
        // update_block
        //
        // # --------------------------------------------------------------------------------

        let block = match response.details {
            notionrs::block::Block::Bookmark { bookmark } => bookmark,
            e => panic!("{:?}", e),
        };

        let builded_block = block
            .url("https://example.com/index.html")
            .caption(vec![])
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
