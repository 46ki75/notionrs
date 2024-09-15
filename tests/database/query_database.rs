mod integration_tests {

    use notionrs::to_json::ToJson;

    #[tokio::test]
    async fn query_database() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();
        let res = client
            .query_database()
            .database_id(database_id)
            .send()
            .await?;
        println!("{}", res.to_json());

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with page_size
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn query_database_page_size() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();
        let res = client
            .query_database()
            .database_id(database_id)
            .page_size(1)
            .send()
            .await?;
        println!("{}", res.to_json());

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with recursive
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn query_database_recursive() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();
        let res = client
            .query_database()
            .database_id(database_id)
            .recursive()
            .send()
            .await?;
        println!("{}", res.to_json());

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with Filter
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn query_database_filter_simple() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::date_before("Created time", "2024-07-01");

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_checkbox() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::checkbox_is_checked("Checkbox"),
            notionrs::filter::Filter::checkbox_is_not_checked("Checkbox"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_date_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::date_after("Created time", "2024-07-01"),
            notionrs::filter::Filter::date_before("Created time", "2024-07-01"),
            notionrs::filter::Filter::date_equals("Created time", "2024-07-01"),
            notionrs::filter::Filter::date_is_empty("Created time"),
            notionrs::filter::Filter::date_is_not_empty("Created time"),
            notionrs::filter::Filter::date_next_month("Created time"),
            notionrs::filter::Filter::date_next_week("Created time"),
            notionrs::filter::Filter::date_next_year("Created time"),
            notionrs::filter::Filter::date_on_or_after("Created time", "2024-07-01"),
            notionrs::filter::Filter::date_on_or_before("Created time", "2024-07-01"),
            notionrs::filter::Filter::date_past_month("Created time"),
            notionrs::filter::Filter::date_past_week("Created time"),
            notionrs::filter::Filter::date_past_year("Created time"),
            notionrs::filter::Filter::date_this_week("Created time"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_files_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::files_is_empty("Files & media"),
            notionrs::filter::Filter::files_is_not_empty("Files & media"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    // TODO: TEST formula

    #[tokio::test]
    async fn query_database_filter_multi_select_filter() -> Result<(), notionrs::error::NotionError>
    {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::multi_select_contains("Multi-select", "0"),
            notionrs::filter::Filter::multi_select_does_not_contain("Multi-select", "0"),
            notionrs::filter::Filter::multi_select_is_empty("Multi-select"),
            notionrs::filter::Filter::multi_select_is_not_empty("Multi-select"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_number_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::number_does_not_equal("Number", 20),
            notionrs::filter::Filter::number_equals("Number", 20),
            notionrs::filter::Filter::number_greater_than("Number", 20),
            notionrs::filter::Filter::number_greater_than_or_equal_to("Number", 20),
            notionrs::filter::Filter::number_is_empty("Number"),
            notionrs::filter::Filter::number_is_not_empty("Number"),
            notionrs::filter::Filter::number_less_than("Number", 20),
            notionrs::filter::Filter::number_less_than_or_equal_to("Number", 20),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_people_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::people_contains(
                "User",
                "c3abcbc1-126b-42b2-975f-43e204986ea3",
            ),
            notionrs::filter::Filter::people_does_not_contain(
                "User",
                "c3abcbc1-126b-42b2-975f-43e204986ea3",
            ),
            notionrs::filter::Filter::people_is_empty("User"),
            notionrs::filter::Filter::people_is_not_empty("User"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_phone_number_filter() -> Result<(), notionrs::error::NotionError>
    {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::phone_number_contains("Phone Number", "0"),
            notionrs::filter::Filter::phone_number_does_not_contain("Phone Number", "0"),
            notionrs::filter::Filter::phone_number_does_not_equal("Phone Number", "0"),
            notionrs::filter::Filter::phone_number_ends_with("Phone Number", "0"),
            notionrs::filter::Filter::phone_number_does_not_contain("Phone Number", "0"),
            notionrs::filter::Filter::phone_number_is_empty("Phone Number"),
            notionrs::filter::Filter::phone_number_is_not_empty("Phone Number"),
            notionrs::filter::Filter::phone_number_starts_with("Phone Number", "0"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    // TODO: TEST rollup

    #[tokio::test]
    async fn query_database_filter_rich_text_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::rich_text_contains("Text", "0"),
            notionrs::filter::Filter::rich_text_does_not_contain("Text", "0"),
            notionrs::filter::Filter::rich_text_does_not_equal("Text", "0"),
            notionrs::filter::Filter::rich_text_ends_with("Text", "0"),
            notionrs::filter::Filter::rich_text_does_not_contain("Text", "0"),
            notionrs::filter::Filter::rich_text_is_empty("Text"),
            notionrs::filter::Filter::rich_text_is_not_empty("Text"),
            notionrs::filter::Filter::rich_text_starts_with("Text", "0"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_select_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::select_does_not_equal("Select", "0"),
            notionrs::filter::Filter::select_equals("Select", "0"),
            notionrs::filter::Filter::select_is_empty("Select"),
            notionrs::filter::Filter::select_is_not_empty("Select"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_status_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::status_does_not_equal("Status", "0"),
            notionrs::filter::Filter::status_equals("Status", "0"),
            notionrs::filter::Filter::status_is_empty("Status"),
            notionrs::filter::Filter::status_is_not_empty("Status"),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_timestamp_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::timestamp_after("2024-07-01"),
            notionrs::filter::Filter::timestamp_before("2024-07-01"),
            notionrs::filter::Filter::timestamp_equals("2024-07-01"),
            notionrs::filter::Filter::timestamp_is_empty(),
            notionrs::filter::Filter::timestamp_is_not_empty(),
            notionrs::filter::Filter::timestamp_next_month(),
            notionrs::filter::Filter::timestamp_next_week(),
            notionrs::filter::Filter::timestamp_next_year(),
            notionrs::filter::Filter::timestamp_on_or_after("2024-07-01"),
            notionrs::filter::Filter::timestamp_on_or_before("2024-07-01"),
            notionrs::filter::Filter::timestamp_past_month(),
            notionrs::filter::Filter::timestamp_past_week(),
            notionrs::filter::Filter::timestamp_past_year(),
            notionrs::filter::Filter::timestamp_this_week(),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }

    #[tokio::test]
    async fn query_database_filter_unique_id_filter() -> Result<(), notionrs::error::NotionError> {
        dotenvy::dotenv().ok();
        let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let filter = notionrs::filter::Filter::or(vec![
            notionrs::filter::Filter::unique_id_does_not_equal("ID", 20),
            notionrs::filter::Filter::unique_id_equals("ID", 20),
            notionrs::filter::Filter::unique_id_greater_than("ID", 20),
            notionrs::filter::Filter::unique_id_greater_than_or_equal_to("ID", 20),
            notionrs::filter::Filter::unique_id_less_than("ID", 20),
            notionrs::filter::Filter::unique_id_less_than_or_equal_to("ID", 20),
        ]);

        let request = client
            .query_database()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }
}