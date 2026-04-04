mod integration_tests {

    use crate::r#trait::PaginateExt;
    use futures::TryStreamExt;

    // # --------------------------------------------------------------------------------
    //
    // list_custom_emojis
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn list_custom_emojis() -> Result<(), crate::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).ok();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = crate::Client::new(notion_api_key);

        let response = client.list_custom_emojis().send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // list_custom_emojis (paginated / collect all)
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn list_custom_emojis_all() -> Result<(), crate::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).ok();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = crate::Client::new(notion_api_key);

        let results: Vec<notionrs_types::prelude::CustomEmojiContent> = client
            .list_custom_emojis()
            .into_stream()
            .try_collect()
            .await?;

        println!("{}", serde_json::to_string(&results)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // list_custom_emojis filtered by name
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn list_custom_emojis_by_name() -> Result<(), crate::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).ok();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = crate::Client::new(notion_api_key);

        let response = client
            .list_custom_emojis()
            .name("vscode".to_string())
            .send()
            .await?;

        println!("{}", serde_json::to_string(&response)?);

        // All returned emojis must match the requested name.
        for emoji in &response.results {
            assert_eq!(emoji.name, "vscode");
        }

        Ok(())
    }
}
