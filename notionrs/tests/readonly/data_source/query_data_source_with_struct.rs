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
        #[serde(rename = "ID")]
        pub id: PageUniqueIdProperty,

        #[serde(rename = "My Title")]
        pub my_title: PageTitleProperty,

        #[serde(rename = "Text")]
        pub text: PageRichTextProperty,

        #[serde(rename = "URL")]
        pub url: PageUrlProperty,

        #[serde(rename = "API Type")]
        pub api_type: PageSelectProperty,

        #[serde(rename = "Status")]
        pub status: PageStatusProperty,

        #[serde(rename = "Multi-select")]
        pub multi_select: PageMultiSelectProperty,

        #[serde(rename = "Checkbox")]
        pub checkbox: PageCheckboxProperty,

        #[serde(rename = "My Date")]
        pub my_date: PageDateProperty,

        #[serde(rename = "User")]
        pub user: PagePeopleProperty,

        #[serde(rename = "Phone Number")]
        pub phone_number: PagePhoneNumberProperty,

        #[serde(rename = "Files & media")]
        pub files_and_media: PageFilesProperty,

        #[serde(rename = "Number")]
        pub number: PageNumberProperty,

        #[serde(rename = "Related Read-only: Integration Test")]
        pub related_read_only_integration_test: PageRelationProperty,

        #[serde(rename = "Related back to Read-only: Integration Test")]
        pub related_back_to_read_only_integration_test: PageRelationProperty,

        #[serde(rename = "Rollup")]
        pub rollup: PageRollupProperty,

        #[serde(rename = "Place")]
        pub place: PagePlaceProperty,

        #[serde(rename = "Created time")]
        pub created_time: PageCreatedTimeProperty,

        #[serde(rename = "Updated time")]
        pub updated_time: PageLastEditedTimeProperty,

        #[serde(rename = "Created by")]
        pub updated_by: PageCreatedByProperty,

        #[serde(rename = "Last edited by")]
        pub last_edited_by: PageLastEditedByProperty,
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
