mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // query_data_source_all (struct)
    //
    // # --------------------------------------------------------------------------------

    use futures::TryStreamExt;
    use notionrs::r#trait::PaginateExt;

    #[tokio::test]
    async fn query_data_source_all_with_struct() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .into_stream()
            .try_collect()
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with page_size
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn query_data_source_all_with_struct_page_size() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .page_size(10)
            .into_stream()
            .try_collect()
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with Filter
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_simple() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let filter = notionrs_types::object::request::filter::Filter::date_before(
            "Created time",
            "2024-07-01",
        );

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_checkbox() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::checkbox_is_checked("Checkbox"),
            notionrs_types::object::request::filter::Filter::checkbox_is_not_checked("Checkbox"),
        ]);

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_date_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_files_filter() -> Result<(), notionrs::Error>
    {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::files_is_empty("Files & media"),
            notionrs_types::object::request::filter::Filter::files_is_not_empty("Files & media"),
        ]);

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_multi_select_filter()
    -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_number_filter() -> Result<(), notionrs::Error>
    {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_people_filter() -> Result<(), notionrs::Error>
    {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_phone_number_filter()
    -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_relation_filter()
    -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::relation_contains(
                "Related Read-only: Integration Test",
                "9804c957-5566-4a9d-b37d-c554bef54e7a",
            ),
            notionrs_types::object::request::filter::Filter::relation_does_not_contain(
                "Related Read-only: Integration Test",
                "9804c957-5566-4a9d-b37d-c554bef54e7a",
            ),
            notionrs_types::object::request::filter::Filter::relation_is_empty(
                "Related Read-only: Integration Test",
            ),
            notionrs_types::object::request::filter::Filter::relation_is_not_empty(
                "Related Read-only: Integration Test",
            ),
        ]);

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_rollup_filter() -> Result<(), notionrs::Error>
    {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_rich_text_filter()
    -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_select_filter() -> Result<(), notionrs::Error>
    {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let filter = notionrs_types::object::request::filter::Filter::or(vec![
            notionrs_types::object::request::filter::Filter::select_does_not_equal(
                "API Type", "Block",
            ),
            notionrs_types::object::request::filter::Filter::select_equals("API Type", "Block"),
            notionrs_types::object::request::filter::Filter::select_is_empty("API Type"),
            notionrs_types::object::request::filter::Filter::select_is_not_empty("API Type"),
        ]);

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_timestamp_filter()
    -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn query_data_source_all_with_struct_filter_unique_id_filter()
    -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .filter(filter)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // working with Sort
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn query_data_source_all_with_struct_sort() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let sorts = vec![notionrs_types::object::request::sort::Sort::asc(
            "Created time",
        )];

        let response: Vec<
            notionrs_types::object::page::PageResponse<
                crate::data_source_schema::IntegrationTestDataSourceSchema,
            >,
        > = client
            .query_data_source()
            .typed::<crate::data_source_schema::IntegrationTestDataSourceSchema>()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .sorts(sorts)
            .into_stream()
            .try_collect()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
