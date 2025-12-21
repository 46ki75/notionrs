mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn move_page() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let parent_page_id =
            std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // create a page to move

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("source page")),
        );

        let request = client
            .create_page()
            .properties(properties)
            .page_id(&parent_page_id)
            .icon(notionrs_types::object::icon::Icon::Emoji(
                notionrs_types::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs_types::object::file::File::External(
                notionrs_types::object::file::ExternalFile::from(
                    "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
                ),
            ));

        let source_page = request.send().await?;

        // crate a target page to move to

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("target page")),
        );

        let request = client
            .create_page()
            .properties(properties)
            .page_id(&parent_page_id)
            .icon(notionrs_types::object::icon::Icon::Emoji(
                notionrs_types::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs_types::object::file::File::External(
                notionrs_types::object::file::ExternalFile::from(
                    "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
                ),
            ));

        let destination_page = request.send().await?;

        // move the source page to the target page

        let request = client
            .move_page()
            .source_page_id(&source_page.id)
            .destination_page_id(&destination_page.id);

        let _moved_page = request.send().await?;

        // cleanup

        let request = client.delete_block().block_id(&destination_page.id);
        let _response = request.send().await?;

        Ok(())
    }
}
