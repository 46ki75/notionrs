mod integration_tests {

    use notionrs_types::prelude::*;

    // # --------------------------------------------------------------------------------
    //
    // query_data_source (struct)
    //
    // # --------------------------------------------------------------------------------

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct MyCrateProperties {
        #[serde(rename = "My Title")]
        pub my_title: PageTitleProperty,
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn query_data_source_with_struct() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let data_source_id =
            std::env::var("NOTION_IT_DATA_SOURCE_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let res = client
            .query_data_source()
            .data_source_id(data_source_id)
            .send::<MyCrateProperties>()
            .await?;

        println!("{}", serde_json::to_string(&res)?);

        Ok(())
    }
}
