// `create_view_query` + `get_view_query_results` + `delete_view_query`
// Those APIs look like mutable operations, but they are actually read-only operations.
// They are used to query the content of a view, and the query results are not stored in Notion.
// The `query_id` is just an identifier for the query,
// and it does not have any side effects on the data in Notion. Therefore,
// it is safe to use those APIs in read-only mode.

mod integration_tests {

    /// <https://www.notion.so/33ea03d79b2680c9a869fd22af127b56?v=33ea03d79b268028873a000c099dd218>
    static VIEW_ID: &str = "33ea03d79b268028873a000c099dd218";

    #[tokio::test]
    async fn crud_view_query() {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();

        let client = notionrs::Client::new(notion_api_key);

        // `create_view_query`

        let results = client
            .create_view_query()
            .view_id(VIEW_ID)
            .send()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&results).unwrap());

        // `get_view_query_results`

        let query_id = results.id;

        let results = client
            .get_view_query_results()
            .view_id(VIEW_ID)
            .query_id(&query_id)
            .send()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&results).unwrap());

        // `delete_view_query`

        let results = client
            .delete_view_query()
            .view_id(VIEW_ID)
            .query_id(&query_id)
            .send()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&results).unwrap());
    }
}
