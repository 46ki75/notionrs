mod integration_tests {

    use notionrs::to_json::ToJson;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    ///
    /// To conduct integration testing, write the following in the `.env` file.
    /// ```ini
    /// NOTION_PAGE_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    /// ```
    #[tokio::test]
    async fn get_page() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let page_id = std::env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::Client::new();

        let request = client.get_page().page_id(page_id);

        let response = request.send().await?;

        let id_property = response.properties.get("ID").unwrap();

        let unique_id = match id_property {
            notionrs::page::properties::PageProperty::UniqueId(i) => i.unique_id.number,
            _ => todo!(),
        };

        println!("ID is {}", unique_id);

        println!("{}", response.to_json());

        Ok(())
    }
}
