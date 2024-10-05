mod integration_tests {

    #[tokio::test]
    async fn crud_audio_block() -> Result<(), notionrs::error::NotionError> {
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
            .children(vec![notionrs::block::Block::new_audio()
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

        // # --------------------------------------------------------------------------------
        //
        // update_block
        //
        // # --------------------------------------------------------------------------------

        let audio_block = match response.details {
            notionrs::block::Block::Audio(audio) => audio,
            e => panic!("{:?}", e),
        };

        let builded_audio_block = audio_block.url("https://example.com/foobar.wav").build();

        let request = client
            .update_block()
            .block_id(response.id.clone())
            .block(builded_audio_block);

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
