mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b26809fb5ede7cf64f55547>
    static PAGE_ID: &str = "33da03d79b26809fb5ede7cf64f55547";

    #[tokio::test]
    async fn crud_to_do_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = RichText::from("list item");

        let block = Block::ToDo {
            to_do: ToDoBlock::default()
                .rich_text(vec![rich_text.clone()])
                .checked(true),
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
            Block::ToDo { to_do } => {
                assert_eq!(to_do.rich_text, vec![rich_text]);
                assert_eq!(to_do.color, notionrs_types::object::color::Color::Default);
                Block::ToDo {
                    to_do: to_do.green_background(),
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
