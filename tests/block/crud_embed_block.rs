mod integration_tests {

    #[tokio::test]
    async fn crud_embed_block() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let block = notionrs::block::Block::Embed {
            embed: notionrs::block::EmbedBlock::new().url("https://example.com"),
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

        let block = match response.block {
            notionrs::block::Block::Embed { embed } => {
                assert_eq!(embed.url, "https://example.com");
                notionrs::block::Block::Embed {
                    embed: embed.url("https://example.com/index.html"),
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