mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b26803fadd3e20967b9a294>
    static PAGE_ID: &str = "33da03d79b26803fadd3e20967b9a294";

    #[tokio::test]
    async fn crud_tab_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        fn generate_paragraph_block(tab_name: &str, text: &str) -> Block {
            Block::Paragraph {
                paragraph: ParagraphBlock {
                    rich_text: vec![RichText::from(tab_name)],
                    children: Some(vec![Block::Paragraph {
                        paragraph: ParagraphBlock {
                            rich_text: vec![RichText::from(text)],
                            ..Default::default()
                        },
                    }]),
                    ..Default::default()
                },
            }
        }

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let children = vec![
            generate_paragraph_block("tab1", "tab 1 content"),
            generate_paragraph_block("tab2", "tab 2 content"),
            generate_paragraph_block("tab3", "tab 3 content"),
        ];

        let tab_block = Block::Tab {
            tab: TabBlock {
                children: Some(children),
            },
        };

        let request = client
            .append_block_children()
            .block_id(PAGE_ID)
            .children(vec![tab_block]);

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
        // delete_block
        //
        // # --------------------------------------------------------------------------------

        let request = client.delete_block().block_id(response.id.clone());

        let response = request.send().await?;

        println!("{:?}", response);

        Ok(())
    }
}
