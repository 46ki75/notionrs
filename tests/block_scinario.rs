mod integration_tests {
    use notionrs::to_json::ToJson;

    #[tokio::test]
    async fn block_scinario() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        // # --------------------------------------------------------------------------------
        //
        // get_block
        //
        // # --------------------------------------------------------------------------------

        let request = client.get_block().block_id(block_id.clone());

        let response = request.send().await?;

        println!("{:?}", response.to_json());

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        // bookmark

        let mut blocks: Vec<notionrs::block::BlockType> = vec![];

        let rich_text = notionrs::others::rich_text::RichText::from("Rich Text");

        blocks.push(
            notionrs::block::BlockType::bookmark("https://example.com")
                .caption(vec![])
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::callout()
                .rich_text(vec![rich_text.clone()])
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::code()
                .rich_text(vec![rich_text.clone()])
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::embed()
                .url("https://example.com")
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::file()
                .url("https://example.com/file.txt")
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::heading_1()
                .rich_text(vec![rich_text.clone()])
                .build_heading_1(),
        );

        blocks.push(
            notionrs::block::BlockType::heading_2()
                .rich_text(vec![rich_text.clone()])
                .build_heading_2(),
        );

        blocks.push(
            notionrs::block::BlockType::heading_3()
                .rich_text(vec![rich_text.clone()])
                .build_heading_3(),
        );

        blocks.push(
            notionrs::block::BlockType::image()
                .url("https://fastly.picsum.photos/id/938/200/300.jpg?hmac=MVXKrDXBUPK5fv_Ev3FTdCFeYf9rvJE2Tz9xynjeelM")
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::numbered_list_tem()
                .rich_text(vec![rich_text.clone()])
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::paragraph()
                .rich_text(vec![rich_text.clone()])
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::quote()
                .rich_text(vec![rich_text.clone()])
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::to_do()
                .rich_text(vec![rich_text.clone()])
                .build(),
        );

        blocks.push(
            notionrs::block::BlockType::toggle()
                .rich_text(vec![rich_text.clone()])
                .build(),
        );

        let request = client
            .append_block_children()
            .block_id(block_id.clone())
            .children(blocks);

        let response = request.send().await?;

        println!("{:?}", response.to_json());

        // # --------------------------------------------------------------------------------
        //
        // get_block_children
        //
        // # --------------------------------------------------------------------------------

        let request = client
            .get_block_children()
            .block_id(block_id.clone())
            .recursive();

        let response = request.send().await?;

        println!("{:?}", response.to_json());

        // # --------------------------------------------------------------------------------
        //
        // delete_block
        //
        // # --------------------------------------------------------------------------------

        let block_id_list = response
            .results
            .into_iter()
            .map(|block| block.id)
            .collect::<Vec<String>>();

        for id in block_id_list {
            let request = client.delete_block().block_id(id);
            let response = request.send().await?;
            println!("{:?}", response.to_json());
        }

        Ok(())
    }
}
