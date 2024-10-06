mod integration_tests {

    #[tokio::test]
    async fn crud_code_block() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = notionrs::RichText::from("console.log(0)");

        let caption = notionrs::RichText::from("index.js");

        let block = notionrs::block::Block::Code {
            code: notionrs::block::CodeBlock::new()
                .rich_text(vec![rich_text.clone()])
                .caption(vec![caption.clone()])
                .lnaguage(notionrs::others::language::Language::Javascript),
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
            notionrs::block::Block::Code { code } => {
                assert_eq!(code.rich_text, vec![rich_text]);
                assert_eq!(code.caption, vec![caption]);
                assert_eq!(
                    code.language,
                    notionrs::others::language::Language::Javascript
                );
                notionrs::block::Block::Code {
                    code: code
                        .lnaguage(notionrs::others::language::Language::Typescript)
                        .caption(vec![notionrs::others::rich_text::RichText::from(
                            "index.ts",
                        )]),
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
