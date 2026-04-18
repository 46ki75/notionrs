mod integration_tests {

    /// <https://www.notion.so/33da03d79b268023aeb5ca96c7d8ffe0>
    static PAGE_ID: &str = "33da03d79b268023aeb5ca96c7d8ffe0";

    #[tokio::test]
    async fn delete_comment() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // First, create a comment so we have one to delete
        let rich_text = vec![notionrs_types::prelude::RichText::from("Comment to delete")];
        let create_response = client
            .create_comment()
            .page_id(PAGE_ID)
            .rich_text(rich_text)
            .send()
            .await?;

        let comment_id = create_response.id.clone();

        // Now delete it
        let response = client
            .delete_comment()
            .comment_id(&comment_id)
            .send()
            .await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
