mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn crud_numbered_list_item_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env"))
            .expect("Failed to load ../.env file");

        let block_id = std::env::var("NOTION_IT_CRUD_PAGE_ID").unwrap();

                let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = RichText::from("list item");

        let block = Block::NumberedListItem {
            numbered_list_item: NumberedListItemBlock::default()
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
            Block::NumberedListItem { numbered_list_item } => {
                assert_eq!(numbered_list_item.rich_text, vec![rich_text]);
                assert_eq!(
                    numbered_list_item.color,
                    notionrs_types::object::color::Color::BlueBackground
                );
                Block::NumberedListItem {
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
