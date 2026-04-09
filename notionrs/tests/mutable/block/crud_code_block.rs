mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b2680eeaff4f90b8b5e900c>
    static PAGE_ID: &str = "33da03d79b2680eeaff4f90b8b5e900c";

    #[tokio::test]
    async fn crud_code_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = RichText::from("console.log(0)");

        let caption = RichText::from("index.js");

        let block = Block::Code {
            code: CodeBlock::default()
                .rich_text(vec![rich_text.clone()])
                .caption(vec![caption.clone()])
                .lnaguage(notionrs_types::object::language::Language::Javascript),
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
            Block::Code { code } => {
                assert_eq!(code.rich_text, vec![rich_text]);
                assert_eq!(code.caption, vec![caption]);
                assert_eq!(
                    code.language,
                    notionrs_types::object::language::Language::Javascript
                );
                Block::Code {
                    code: code
                        .lnaguage(notionrs_types::object::language::Language::Typescript)
                        .caption(vec![RichText::from("index.ts")]),
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
