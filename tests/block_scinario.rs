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

        let mut blocks = vec![notionrs::block::BlockType::bookmark("https://example.com").build()];

        blocks.push(
            notionrs::block::BlockType::file()
                .url("https://companywebsite.com/files/doc.txt")
                .name("My TXT File")
                .build(),
        );

        // bulleted_list_item

        // blocks.push(
        //     notionrs::request_builder::block::BlockRequest::bulleted_list_item()
        //         .rich_text(vec![
        //             notionrs::request_builder::rich_text::RichTextRequest::new(
        //                 "const message = 'Hello, Rust!'",
        //             ),
        //         ])
        //         .build(),
        // );

        // blocks.push(
        //     notionrs::request_builder::block::BlockRequest::callout()
        //         .rich_text(vec![
        //             notionrs::request_builder::rich_text::RichTextRequest::new(
        //                 "const message = 'Hello, Rust!'",
        //             ),
        //         ])
        //         .build(),
        // );

        // // code

        // blocks.push(
        //     notionrs::request_builder::block::BlockRequest::code()
        //         .rich_text(vec![
        //             notionrs::request_builder::rich_text::RichTextRequest::new(
        //                 "const message = 'Hello, Rust!'",
        //             ),
        //         ])
        //         .language(notionrs::others::language::Language::Typescript)
        //         .build(),
        // );

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
