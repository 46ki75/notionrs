mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // get_async_task
    //
    // # --------------------------------------------------------------------------------

    /// There is no way to deterministically obtain a real async task ID without
    /// triggering a mutable, asynchronous operation, so this only exercises the
    /// endpoint's error path (an unknown task ID should fail, not panic or hang).
    #[tokio::test]
    async fn get_async_task_not_found() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let result = client
            .get_async_task()
            .task_id("00000000-0000-0000-0000-000000000000")
            .send()
            .await;

        assert!(result.is_err());

        match result.unwrap_err() {
            notionrs::Error::Http { status, .. } => {
                assert!((400..500).contains(&status), "unexpected status: {status}");
            }
            other => panic!("Expected Http error, got: {:?}", other),
        }

        Ok(())
    }
}
