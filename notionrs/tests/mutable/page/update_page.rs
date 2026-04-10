mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn update_page_with_template_default() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // create_page (to get a page to update)
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "My Title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title")),
        );

        let created = client
            .create_page()
            .properties(properties)
            .data_source_id(crate::mutable::DATA_SOURCE_ID)
            .send()
            .await?;

        // # --------------------------------------------------------------------------------
        //
        // update_page with template default
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "My Title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title (Updated)")),
        );

        let _updated = client
            .update_page()
            .page_id(&created.id)
            .properties(properties)
            .template_default()
            .send()
            .await?;

        // # --------------------------------------------------------------------------------
        //
        // cleanup
        //
        // # --------------------------------------------------------------------------------

        let response = client.delete_block().block_id(created.id).send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn update_page_with_template_default_with_timezone() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // create_page (to get a page to update)
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "My Title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title")),
        );

        let created = client
            .create_page()
            .properties(properties)
            .data_source_id(crate::mutable::DATA_SOURCE_ID)
            .send()
            .await?;

        // # --------------------------------------------------------------------------------
        //
        // update_page with template default and timezone
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "My Title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title (Updated)")),
        );

        let _updated = client
            .update_page()
            .page_id(&created.id)
            .properties(properties)
            .template_default_with_timezone("America/New_York".to_string())
            .send()
            .await?;

        // # --------------------------------------------------------------------------------
        //
        // cleanup
        //
        // # --------------------------------------------------------------------------------

        let response = client.delete_block().block_id(created.id).send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
