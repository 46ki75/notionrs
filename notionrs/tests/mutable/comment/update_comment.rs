mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b268023aeb5ca96c7d8ffe0>
    static PAGE_ID: &str = "33da03d79b268023aeb5ca96c7d8ffe0";

    #[tokio::test]
    async fn update_comment_rich_text() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // First, create a comment so we have one to update
        let rich_text = vec![RichText::from("Comment to update")];
        let create_response = client
            .create_comment()
            .page_id(PAGE_ID)
            .rich_text(rich_text)
            .send()
            .await?;

        let comment_id = create_response.id.clone();

        // Now update it
        let updated_text = vec![RichText::from("Updated comment text")];
        let response = client
            .update_comment()
            .comment_id(&comment_id)
            .rich_text(updated_text)
            .send()
            .await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn update_comment_markdown() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // First, create a comment so we have one to update
        let rich_text = vec![RichText::from("Comment to update with markdown")];
        let create_response = client
            .create_comment()
            .page_id(PAGE_ID)
            .rich_text(rich_text)
            .send()
            .await?;

        let comment_id = create_response.id.clone();

        // Now update it with markdown
        let response = client
            .update_comment()
            .comment_id(&comment_id)
            .markdown("Updated with **markdown**")
            .send()
            .await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
