mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn crud_tab_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let block_id = std::env::var("NOTION_IT_CRUD_PAGE_ID").unwrap();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
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
            .block_id(block_id.clone())
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
