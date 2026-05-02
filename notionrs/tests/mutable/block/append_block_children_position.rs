mod integration_tests {
    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn append_block_children_with_position() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // create parent block
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "My Title".to_string(),
            PageProperty::Title(PageTitleProperty::from("Parent Page for test")),
        );

        let pre_page = client
            .create_page::<std::collections::HashMap<String, PageProperty>>()
            .properties(properties)
            .data_source_id(crate::mutable::DATA_SOURCE_ID)
            .send()
            .await?;

        // Add an initial block
        let initial_block = Block::Paragraph {
            paragraph: ParagraphBlock::from("Initial Block"),
        };
        let response = client
            .append_block_children()
            .block_id(&pre_page.id)
            .children(vec![initial_block])
            .send()
            .await?;

        let initial_block_id = &response.results[0].id;

        // Position: Start
        let start_block = Block::Paragraph {
            paragraph: ParagraphBlock::from("Start Block"),
        };
        let _ = client
            .append_block_children()
            .block_id(&pre_page.id)
            .children(vec![start_block])
            .position_start()
            .send()
            .await?;

        // Position: End
        let end_block = Block::Paragraph {
            paragraph: ParagraphBlock::from("End Block"),
        };
        let _ = client
            .append_block_children()
            .block_id(&pre_page.id)
            .children(vec![end_block])
            .position_end()
            .send()
            .await?;

        // Position: After Block
        let after_block = Block::Paragraph {
            paragraph: ParagraphBlock::from("After Initial Block"),
        };
        let _ = client
            .append_block_children()
            .block_id(&pre_page.id)
            .children(vec![after_block])
            .position_after_block(initial_block_id)
            .send()
            .await?;

        // Cleanup
        let _ = client.delete_block().block_id(pre_page.id).send().await?;

        Ok(())
    }
}
