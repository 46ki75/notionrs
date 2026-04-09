mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b2680a2a80ecfc632be5af1>
    static PAGE_ID: &str = "33da03d79b2680a2a80ecfc632be5af1";

    #[tokio::test]
    async fn crud_synced_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        // origin

        let block = Block::Bookmark {
            bookmark: BookmarkBlock::default().url("https://example.com"),
        };

        let children = vec![block];

        let block = Block::SyncedBlock {
            synced_block: SyncedBlock::default().children(children),
        };

        let request = client
            .append_block_children()
            .block_id(PAGE_ID)
            .children(vec![block]);

        let response = request.send().await?;

        let origin_block_id = response.results.first().unwrap().id.clone();

        // sync

        let block = Block::SyncedBlock {
            synced_block: SyncedBlock::default()
                .block_id(response.results.first().unwrap().id.clone()),
        };

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

        let _ = request.send().await?;

        let request = client.delete_block().block_id(origin_block_id);

        let response = request.send().await?;

        println!("{:?}", response);

        Ok(())
    }
}
