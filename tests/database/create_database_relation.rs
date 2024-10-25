mod integration_tests {

    #[tokio::test]
    async fn create_database_relation() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let page_id = std::env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::Client::new();

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            notionrs::database::DatabaseProperty::Title(
                notionrs::database::DatabaseTitleProperty::default(),
            ),
        );

        let request = client
            .create_database()
            .page_id(page_id.clone())
            .title(vec![notionrs::RichText::from("Database Title")])
            .properties(properties);

        let response = request.send().await?;

        if let notionrs::database::DatabaseProperty::Title(title) =
            response.properties.get("Title").unwrap()
        {
            let mut properties = std::collections::HashMap::new();

            let database_id = response.id;

            properties.insert(
                "Title".to_string(),
                notionrs::database::DatabaseProperty::Title(
                    notionrs::database::DatabaseTitleProperty::default(),
                ),
            );

            properties.insert(
                "One-WayRelation".to_string(),
                notionrs::database::DatabaseProperty::Relation(
                    notionrs::database::DatabaseRelationProperty::create_one_way_relation(
                        database_id.clone(),
                    ),
                ),
            );

            properties.insert(
                "Two-Way-Relation".to_string(),
                notionrs::database::DatabaseProperty::Relation(
                    notionrs::database::DatabaseRelationProperty::create_tow_way_relation(
                        database_id,
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
        }

        Ok(())
    }
}
