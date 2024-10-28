mod integration_tests {

    #[tokio::test]
    async fn crud_numbered_list_item_block() -> Result<(), notionrs::error::Error> {
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

        let rich_text = notionrs::RichText::from("list item");

        let block = notionrs::block::Block::NumberedListItem {
            numbered_list_item: notionrs::block::NumberedListItemBlock::new()
                .rich_text(vec![rich_text.clone()])
                .blue_background(),
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
            notionrs::block::Block::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text, vec![rich_text]);
                assert_eq!(
                    numbered_list_item.color,
                    notionrs::others::color::Color::BlueBackground
                );
                notionrs::block::Block::NumberedListItem {
                    numbered_list_item: numbered_list_item.green_background(),
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
