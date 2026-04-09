mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b2680a2a80ecfc632be5af1>
    static PAGE_ID: &str = "33da03d79b2680a2a80ecfc632be5af1";

    #[tokio::test]
    async fn create_comment() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
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
}
