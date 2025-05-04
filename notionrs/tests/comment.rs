mod integration_tests {

    use notionrs::prelude::*;

    // # --------------------------------------------------------------------------------
    //
    // create_comment
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let rich_text = vec![RichText::from("Test Comment!")];

        let request = client
            .create_comment()
            .page_id(page_id)
            .rich_text(rich_text);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // retrieve_comments
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn retrieve_comments() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let request = client.retrieve_comments().block_id(page_id);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
