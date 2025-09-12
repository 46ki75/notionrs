mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn create_database_relation() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            DataSourceProperty::Title(DataSourceTitleProperty::default()),
        );

        let request = client
            .create_database()
            .page_id(page_id.clone())
            .title(vec![RichText::from("Database Title")])
            .properties(properties);

        let response = request.send().await?;

        let _database_id = response.id;

        // if let DatabaseProperty::Title(title) = response.properties.get("Title").unwrap() {
        //     let mut properties = std::collections::HashMap::new();

        //     properties.insert(
        //         "Title".to_string(),
        //         DatabaseProperty::Title(DatabaseTitleProperty::default()),
        //     );

        //     properties.insert(
        //         "One-WayRelation".to_string(),
        //         DatabaseProperty::Relation(DatabaseRelationProperty::create_one_way_relation(
        //             database_id.clone(),
        //         )),
        //     );

        //     properties.insert(
        //         "Two-Way-Relation".to_string(),
        //         DatabaseProperty::Relation(DatabaseRelationProperty::create_tow_way_relation(
        //             database_id.clone(),
        //             title.clone().id.unwrap(),
        //             title.clone().name,
        //         )),
        //     );

        //     let request = client
        //         .create_database()
        //         .page_id(page_id)
        //         .title(vec![RichText::from("Database Title")])
        //         .properties(properties);

        //     let response = request.send().await?;

        //     println!("{}", serde_json::to_string(&response).unwrap());

        //     // delete relation database
        //     let request = client.delete_block().block_id(response.id);

        //     let _ = request.send().await?;

        //     // delete origin database
        //     let request = client.delete_block().block_id(database_id.clone());

        //     let _ = request.send().await?;
        // }

        Ok(())
    }
}
