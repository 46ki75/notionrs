mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b2680188a9cc2cdb395d4b1>
    static PAGE_ID: &str = "33da03d79b2680188a9cc2cdb395d4b1";

    #[tokio::test]
    async fn crud_audio_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let block = Block::Audio {
            audio: notionrs_types::object::file::File::default()
                .url("https://example.com/sample.wav")
                .caption(vec![RichText::from("my caption")]),
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
        // update_block
        //
        // # --------------------------------------------------------------------------------

        let block = match response.block {
            Block::Audio { audio } => {
                assert_eq!(audio.get_url(), "https://example.com/sample.wav");
                Block::Audio {
                    audio: audio.url("https://example.com/foobar.wav"),
                }
            }
            e => panic!("{:?}", e),
        };

        let request = client
            .update_block()
            .block_id(response.id.clone())
            .block(block);

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
