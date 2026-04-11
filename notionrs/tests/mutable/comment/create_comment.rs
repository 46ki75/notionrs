mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b268023aeb5ca96c7d8ffe0>
    static PAGE_ID: &str = "33da03d79b268023aeb5ca96c7d8ffe0";

    #[tokio::test]
    async fn create_comment() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let rich_text = vec![RichText::from("Test Comment!")];

        let request = client
            .create_comment()
            .page_id(PAGE_ID)
            .rich_text(rich_text);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn create_comment_markdown() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client
            .create_comment()
            .page_id(PAGE_ID)
            .markdown("I'm **bald** 😎");

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
