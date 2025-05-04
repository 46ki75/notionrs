mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // query_database_all
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();
        let response = client
            .query_database_all()
            .database_id(database_id)
            .send()
            .await?;
        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with page_size
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_page_size() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();
        let response = client
            .query_database_all()
            .database_id(database_id)
            .send()
            .await?;
        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with Filter
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_simple() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::date_before(
            "Created time",
            "2024-07-01",
        );

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_checkbox() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::checkbox_is_checked("Checkbox"),
            notionrs_types::object::request::filter::Filter::checkbox_is_not_checked("Checkbox"),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_date_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::date_after(
                "Created time",
                "2024-07-01",
            ),
            notionrs_types::object::request::filter::Filter::date_before(
                "Created time",
                "2024-07-01",
            ),
            notionrs_types::object::request::filter::Filter::date_equals(
                "Created time",
                "2024-07-01",
            ),
            notionrs_types::object::request::filter::Filter::date_is_empty("Created time"),
            notionrs_types::object::request::filter::Filter::date_is_not_empty("Created time"),
            notionrs_types::object::request::filter::Filter::date_next_month("Created time"),
            notionrs_types::object::request::filter::Filter::date_next_week("Created time"),
            notionrs_types::object::request::filter::Filter::date_next_year("Created time"),
            notionrs_types::object::request::filter::Filter::date_on_or_after(
                "Created time",
                "2024-07-01",
            ),
            notionrs_types::object::request::filter::Filter::date_on_or_before(
                "Created time",
                "2024-07-01",
            ),
            notionrs_types::object::request::filter::Filter::date_past_month("Created time"),
            notionrs_types::object::request::filter::Filter::date_past_week("Created time"),
            notionrs_types::object::request::filter::Filter::date_past_year("Created time"),
            notionrs_types::object::request::filter::Filter::date_this_week("Created time"),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_files_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::files_is_empty("Files & media"),
            notionrs_types::object::request::filter::Filter::files_is_not_empty("Files & media"),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // #[tokio::test]
    // #[serial_test::serial]
    // async fn query_database_all_filter_formula_filter() -> Result<(), notionrs::Error> {
    //     dotenvy::dotenv().ok();
    //     dotenvy::from_path(std::path::Path::new(".env.test"))
    //         .expect("Failed to load .env.test file");

    //     let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

    //     let client = notionrs::Client::new();

    //     let filter = notionrs_types::object::request::filter::Filter::or(vec![
    //         notionrs_types::object::request::filter::Filter::formula_number_does_not_equal("formula", 0),
    //         notionrs_types::object::request::filter::Filter::formula_number_equals("formula", 0),
    //         notionrs_types::object::request::filter::Filter::formula_number_greater_than("formula", 0),
    //         notionrs_types::object::request::filter::Filter::formula_number_greater_than_or_equal("formula", 0),
    //         notionrs_types::object::request::filter::Filter::formula_number_is_empty("formula"),
    //         notionrs_types::object::request::filter::Filter::formula_number_is_not_empty("formula"),
    //         notionrs_types::object::request::filter::Filter::formula_number_less_than("formula", 0),
    //         notionrs_types::object::request::filter::Filter::formula_number_less_than_or_equal("formula", 0),
    //     ]);

    //     let request = client
    //         .query_database_all()
    //         .database_id(database_id)
    //         .filter(filter);

    //     let response = request.send().await?;

    //     println!("{}", serde_json::to_string(&response)?);

    //     Ok(())
    // }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_multi_select_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::multi_select_contains(
                "Multi-select",
                "0",
            ),
            notionrs_types::object::request::filter::Filter::multi_select_does_not_contain(
                "Multi-select",
                "0",
            ),
            notionrs_types::object::request::filter::Filter::multi_select_is_empty("Multi-select"),
            notionrs_types::object::request::filter::Filter::multi_select_is_not_empty(
                "Multi-select",
            ),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_number_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::number_does_not_equal("Number", 20),
            notionrs_types::object::request::filter::Filter::number_equals("Number", 20),
            notionrs_types::object::request::filter::Filter::number_greater_than("Number", 20),
            notionrs_types::object::request::filter::Filter::number_greater_than_or_equal_to(
                "Number", 20,
            ),
            notionrs_types::object::request::filter::Filter::number_is_empty("Number"),
            notionrs_types::object::request::filter::Filter::number_is_not_empty("Number"),
            notionrs_types::object::request::filter::Filter::number_less_than("Number", 20),
            notionrs_types::object::request::filter::Filter::number_less_than_or_equal_to(
                "Number", 20,
            ),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_people_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::people_contains(
                "User",
                "c3abcbc1-126b-42b2-975f-43e204986ea3",
            ),
            notionrs_types::object::request::filter::Filter::people_does_not_contain(
                "User",
                "c3abcbc1-126b-42b2-975f-43e204986ea3",
            ),
            notionrs_types::object::request::filter::Filter::people_is_empty("User"),
            notionrs_types::object::request::filter::Filter::people_is_not_empty("User"),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_phone_number_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::phone_number_contains(
                "Phone Number",
                "0",
            ),
            notionrs_types::object::request::filter::Filter::phone_number_does_not_contain(
                "Phone Number",
                "0",
            ),
            notionrs_types::object::request::filter::Filter::phone_number_does_not_equal(
                "Phone Number",
                "0",
            ),
            notionrs_types::object::request::filter::Filter::phone_number_ends_with(
                "Phone Number",
                "0",
            ),
            notionrs_types::object::request::filter::Filter::phone_number_does_not_contain(
                "Phone Number",
                "0",
            ),
            notionrs_types::object::request::filter::Filter::phone_number_is_empty("Phone Number"),
            notionrs_types::object::request::filter::Filter::phone_number_is_not_empty(
                "Phone Number",
            ),
            notionrs_types::object::request::filter::Filter::phone_number_starts_with(
                "Phone Number",
                "0",
            ),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_relation_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::relation_contains(
                "Relation",
                "9804c957-5566-4a9d-b37d-c554bef54e7a",
            ),
            notionrs_types::object::request::filter::Filter::relation_does_not_contain(
                "Relation",
                "9804c957-5566-4a9d-b37d-c554bef54e7a",
            ),
            notionrs_types::object::request::filter::Filter::relation_is_empty("Relation"),
            notionrs_types::object::request::filter::Filter::relation_is_not_empty("Relation"),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_rollup_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::rollup_any(
                "Rollup",
                notionrs_types::object::request::filter::Filter::rich_text_contains("Title", "a"),
            ),
            notionrs_types::object::request::filter::Filter::rollup_every(
                "Rollup",
                notionrs_types::object::request::filter::Filter::rich_text_contains("Title", "a"),
            ),
            notionrs_types::object::request::filter::Filter::rollup_none(
                "Rollup",
                notionrs_types::object::request::filter::Filter::rich_text_contains("Title", "a"),
            ),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_rich_text_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::rich_text_contains("Text", "0"),
            notionrs_types::object::request::filter::Filter::rich_text_does_not_contain(
                "Text", "0",
            ),
            notionrs_types::object::request::filter::Filter::rich_text_does_not_equal("Text", "0"),
            notionrs_types::object::request::filter::Filter::rich_text_ends_with("Text", "0"),
            notionrs_types::object::request::filter::Filter::rich_text_does_not_contain(
                "Text", "0",
            ),
            notionrs_types::object::request::filter::Filter::rich_text_is_empty("Text"),
            notionrs_types::object::request::filter::Filter::rich_text_is_not_empty("Text"),
            notionrs_types::object::request::filter::Filter::rich_text_starts_with("Text", "0"),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_select_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::select_does_not_equal("Select", "0"),
            notionrs_types::object::request::filter::Filter::select_equals("Select", "0"),
            notionrs_types::object::request::filter::Filter::select_is_empty("Select"),
            notionrs_types::object::request::filter::Filter::select_is_not_empty("Select"),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // #[tokio::test]
    // #[serial_test::serial]
    // async fn query_database_all_filter_status_filter() -> Result<(), notionrs::Error> {
    //     dotenvy::dotenv().ok();
    //     dotenvy::from_path(std::path::Path::new(".env.test"))
    //         .expect("Failed to load .env.test file");

    //     let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

    //     let client = notionrs::Client::new();

    //     let filter = notionrs_types::object::request::filter::Filter::or(vec![
    //         notionrs_types::object::request::filter::Filter::status_does_not_equal("Status", "0"),
    //         notionrs_types::object::request::filter::Filter::status_equals("Status", "0"),
    //         notionrs_types::object::request::filter::Filter::status_is_empty("Status"),
    //         notionrs_types::object::request::filter::Filter::status_is_not_empty("Status"),
    //     ]);

    //     let request = client
    //         .query_database_all()
    //         .database_id(database_id)
    //         .filter(filter);

    //     let response = request.send().await?;

    //     println!("{}", serde_json::to_string(&response)?);

    //     Ok(())
    // }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_timestamp_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::timestamp_after("2024-07-01"),
            notionrs_types::object::request::filter::Filter::timestamp_before("2024-07-01"),
            notionrs_types::object::request::filter::Filter::timestamp_equals("2024-07-01"),
            notionrs_types::object::request::filter::Filter::timestamp_is_empty(),
            notionrs_types::object::request::filter::Filter::timestamp_is_not_empty(),
            notionrs_types::object::request::filter::Filter::timestamp_next_month(),
            notionrs_types::object::request::filter::Filter::timestamp_next_week(),
            notionrs_types::object::request::filter::Filter::timestamp_next_year(),
            notionrs_types::object::request::filter::Filter::timestamp_on_or_after("2024-07-01"),
            notionrs_types::object::request::filter::Filter::timestamp_on_or_before("2024-07-01"),
            notionrs_types::object::request::filter::Filter::timestamp_past_month(),
            notionrs_types::object::request::filter::Filter::timestamp_past_week(),
            notionrs_types::object::request::filter::Filter::timestamp_past_year(),
            notionrs_types::object::request::filter::Filter::timestamp_this_week(),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_filter_unique_id_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::unique_id_does_not_equal("ID", 20),
            notionrs_types::object::request::filter::Filter::unique_id_equals("ID", 20),
            notionrs_types::object::request::filter::Filter::unique_id_greater_than("ID", 20),
            notionrs_types::object::request::filter::Filter::unique_id_greater_than_or_equal_to(
                "ID", 20,
            ),
            notionrs_types::object::request::filter::Filter::unique_id_less_than("ID", 20),
            notionrs_types::object::request::filter::Filter::unique_id_less_than_or_equal_to(
                "ID", 20,
            ),
        ]);

        let request = client
            .query_database_all()
            .database_id(database_id)
            .filter(filter);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with Sort
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn query_database_all_sort() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let sorts = vec![notionrs_types::object::request::sort::Sort::asc(
            "Created time",
        )];

        let request = client
            .query_database_all()
            .database_id(database_id)
            .sorts(sorts);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
