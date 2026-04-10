mod integration_tests {

    /// <https://www.notion.so/33ea03d79b2680e5b1f0f0c7eb889869?v=33ea03d79b2680f7ac11000cd1db055b&source=copy_link>
    static DATABASE_ID: &str = "33ea03d79b2680e5b1f0f0c7eb889869";
    static DATA_SOURCE_ID: &str = "33ea03d7-9b26-8096-b614-000ba583016b";

    #[tokio::test]
    async fn crud_view() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();

        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // create_view
        //
        // # --------------------------------------------------------------------------------

        let created_view = client
            .create_view()
            .data_source_id(DATA_SOURCE_ID)
            .database_id(DATABASE_ID)
            .name("Test View")
            .view_type(notionrs_types::object::view::ViewType::Table)
            .send()
            .await?;

        let created_view_id = &created_view.id;

        assert_eq!(created_view.name, "Test View");
        assert_eq!(
            created_view.r#type,
            notionrs_types::object::view::ViewType::Table
        );

        println!("{}", serde_json::to_string(&created_view).unwrap());

        // # --------------------------------------------------------------------------------
        //
        // list_views
        //
        // # --------------------------------------------------------------------------------

        let list_response = client.list_views().database_id(DATABASE_ID).send().await?;

        assert!(
            list_response
                .results
                .iter()
                .any(|v| &v.id == created_view_id),
            "newly created view should appear in the list"
        );

        println!("{}", serde_json::to_string(&list_response).unwrap());

        // # --------------------------------------------------------------------------------
        //
        // update_view
        //
        // # --------------------------------------------------------------------------------

        let updated_view = client
            .update_view()
            .view_id(created_view_id)
            .name("Test View (Updated)")
            .send()
            .await?;

        assert_eq!(updated_view.name, "Test View (Updated)");

        println!("{}", serde_json::to_string(&updated_view).unwrap());

        // # --------------------------------------------------------------------------------
        //
        // cleanup: delete_view
        //
        // # --------------------------------------------------------------------------------

        let deleted_view = client.delete_view().view_id(created_view_id).send().await?;

        assert_eq!(&deleted_view.id, created_view_id);

        println!("{}", serde_json::to_string(&deleted_view).unwrap());

        Ok(())
    }
}
