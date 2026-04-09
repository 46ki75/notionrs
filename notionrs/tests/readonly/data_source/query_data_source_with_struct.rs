mod integration_tests {

    use notionrs_types::prelude::*;

    static DATA_SOURCE_ID: &str = "33da03d7-9b26-81cb-90c7-000b8fb827a8";

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
    async fn query_data_source_with_struct() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let res = client
            .query_data_source()
            .data_source_id(DATA_SOURCE_ID)
            .send::<MyCrateProperties>()
            .await?;

        println!("{}", serde_json::to_string(&res)?);

        Ok(())
    }
}
