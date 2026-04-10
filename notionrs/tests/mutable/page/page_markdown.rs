mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33ea03d79b268071b2a4e9d5f4393ccd>
    static PAGE_ID: &str = "33ea03d79b268071b2a4e9d5f4393ccd";

    #[tokio::test]
    async fn update_page_markdown_insert_content() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // create a page
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("Markdown Insert Content Test")),
        );

        let page = client
            .create_page()
            .properties(properties)
            .page_id(PAGE_ID)
            .send()
            .await?;

        // insert content using markdown API
        let response = client
            .update_page_markdown()
            .page_id(&page.id)
            .insert_content("# Hello World\n\nThis is inserted content.")
            .send()
            .await?;

        assert_eq!(response.object, "page_markdown");
        assert_eq!(response.id, page.id);
        assert!(!response.markdown.is_empty());

        // cleanup
        client.delete_block().block_id(&page.id).send().await?;

        Ok(())
    }

    #[tokio::test]
    async fn update_page_markdown_replace_content() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // create a page
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("Markdown Replace Content Test")),
        );

        let page = client
            .create_page()
            .properties(properties)
            .page_id(PAGE_ID)
            .send()
            .await?;

        // insert initial content
        client
            .update_page_markdown()
            .page_id(&page.id)
            .insert_content("Initial content to replace.")
            .send()
            .await?;

        // replace entire content
        let response = client
            .update_page_markdown()
            .page_id(&page.id)
            .replace_content("# Replaced\n\nThis is the new content.")
            .send()
            .await?;

        assert_eq!(response.object, "page_markdown");
        assert!(!response.markdown.is_empty());

        // cleanup
        client.delete_block().block_id(&page.id).send().await?;

        Ok(())
    }

    #[tokio::test]
    async fn update_page_markdown_update_content() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // create a page
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("Markdown Update Content Test")),
        );

        let page = client
            .create_page()
            .properties(properties)
            .page_id(PAGE_ID)
            .send()
            .await?;

        // insert initial content
        client
            .update_page_markdown()
            .page_id(&page.id)
            .insert_content("Hello old world.")
            .send()
            .await?;

        // update specific content using search-and-replace
        let content_updates = vec![
            notionrs::client::page::update_page_markdown::ContentUpdate {
                old_str: "old world".to_string(),
                new_str: "new world".to_string(),
                replace_all_matches: None,
            },
        ];

        let response = client
            .update_page_markdown()
            .page_id(&page.id)
            .update_content(content_updates)
            .send()
            .await?;

        assert_eq!(response.object, "page_markdown");
        assert!(response.markdown.contains("new world"));

        // cleanup
        client.delete_block().block_id(&page.id).send().await?;

        Ok(())
    }

    #[tokio::test]
    async fn get_page_markdown() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // create a page
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("Markdown Get Test")),
        );

        let page = client
            .create_page()
            .properties(properties)
            .page_id(PAGE_ID)
            .send()
            .await?;

        // insert content
        client
            .update_page_markdown()
            .page_id(&page.id)
            .insert_content("# Test Heading\n\nSome test content.")
            .send()
            .await?;

        // get the page as markdown
        let response = client.get_page_markdown().page_id(&page.id).send().await?;

        assert_eq!(response.object, "page_markdown");
        assert_eq!(response.id, page.id);
        assert!(!response.markdown.is_empty());

        // cleanup
        client.delete_block().block_id(&page.id).send().await?;

        Ok(())
    }
}
