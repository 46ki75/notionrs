mod integration_tests {

    #[tokio::test]
    async fn create_database_relation() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::Client::new();

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            notionrs::object::database::DatabaseProperty::Title(
                notionrs::object::database::DatabaseTitleProperty::default(),
            ),
        );

        let request = client
            .create_database()
            .page_id(page_id.clone())
            .title(vec![notionrs::RichText::from("Database Title")])
            .properties(properties);

        let response = request.send().await?;

        let database_id = response.id;

        if let notionrs::object::database::DatabaseProperty::Title(title) =
            response.properties.get("Title").unwrap()
        {
            let mut properties = std::collections::HashMap::new();

            properties.insert(
                "Title".to_string(),
                notionrs::object::database::DatabaseProperty::Title(
                    notionrs::object::database::DatabaseTitleProperty::default(),
                ),
            );

            properties.insert(
                "One-WayRelation".to_string(),
                notionrs::object::database::DatabaseProperty::Relation(
                    notionrs::object::database::DatabaseRelationProperty::create_one_way_relation(
                        database_id.clone(),
                    ),
                ),
            );

            properties.insert(
                "Two-Way-Relation".to_string(),
                notionrs::object::database::DatabaseProperty::Relation(
                    notionrs::object::database::DatabaseRelationProperty::create_tow_way_relation(
                        database_id.clone(),
                        title.clone().id.unwrap(),
                        title.clone().name,
                    ),
                ),
            );

            let request = client
                .create_database()
                .page_id(page_id)
                .title(vec![notionrs::RichText::from("Database Title")])
                .properties(properties);

            let response = request.send().await?;

            println!("{}", serde_json::to_string(&response).unwrap());

            // delete relation database
            let request = client.delete_block().block_id(response.id);

            let _ = request.send().await?;

            // delete origin database
            let request = client.delete_block().block_id(database_id.clone());

            let _ = request.send().await?;
        }

        Ok(())
    }
}
