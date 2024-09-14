mod integration_tests {
    // use notionrs::{rich_text, to_json::ToJson};

    #[tokio::test]
    async fn crud_block() -> Result<(), notionrs::error::NotionError> {
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
            .block_id(block_id)
            .children(vec![notionrs::block::BlockType::new_audio()
                .url("https://example.com/sample.wav")
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

        println!("{:?}", response);

        // # --------------------------------------------------------------------------------
        //
        // update_block
        //
        // # --------------------------------------------------------------------------------

        // let request = client
        //     .update_block()
        //     .block_id(response.clone().id)
        //     .block(response);

        // let response = request.send().await?;

        // println!("{:?}", response);

        Ok(())
    }
}
