mod integration_tests {

    use notionrs::to_json::ToJson;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    #[tokio::test]
    async fn get_page_property_item() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let page_id = std::env::var("NOTION_PAGE_ID").unwrap();

        let client = notionrs::client::NotionClient::new();

        let request = client
            .get_page_property_item()
            .page_id(page_id)
            .property_id("%3AlnV");

        let response = request.send().await?;

        match response {
            notionrs::page::properties::PageProperty::Files(f) => {
                println!("{:?}", f.to_json())
            }
            _ => panic!("An unexpected type"),
        }

        Ok(())
    }
}
