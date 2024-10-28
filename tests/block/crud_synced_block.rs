mod integration_tests {

    #[tokio::test]
    async fn crud_synced_block() -> Result<(), notionrs::error::Error> {
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

        // origin

        let block = notionrs::block::Block::Bookmark {
            bookmark: notionrs::block::BookmarkBlock::new().url("https://example.com"),
        };

        let children = vec![block];

        let block = notionrs::block::Block::SyncedBlock {
            synced_block: notionrs::block::SyncedBlock::new().children(children),
        };

        let request = client
            .append_block_children()
            .block_id(block_id.clone())
            .children(vec![block]);

        let response = request.send().await?;

        let origin_block_id = response.results.first().unwrap().id.clone();

        // sync

        let block = notionrs::block::Block::SyncedBlock {
            synced_block: notionrs::block::SyncedBlock::new()
                .block_id(response.results.first().unwrap().id.clone()),
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

        let _ = request.send().await?;

        let request = client.delete_block().block_id(origin_block_id);

        let response = request.send().await?;

        println!("{:?}", response);

        Ok(())
    }
}
