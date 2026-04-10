mod integration_tests {

    /// <https://www.notion.so/33ea03d79b2680e5b1f0f0c7eb889869>
    static DATABASE_ID: &str = "33ea03d79b2680e5b1f0f0c7eb889869";
    /// <https://www.notion.so/33ea03d79b2680e5b1f0f0c7eb889869?v=33ea03d79b2680f7ac11000cd1db055b>
    static VIEW_ID: &str = "33ea03d79b2680f7ac11000cd1db055b";

    #[tokio::test]
    async fn crud_view() {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();

        let client = notionrs::Client::new(notion_api_key);

        // TODO: `create_view`

        // TODO: `list_views`

        // TODO: `update_view`
    }
}
