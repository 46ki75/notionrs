mod integration_tests {

    static PAGE_ID: &str = "33da03d79b268041a2a8c24c0fc00c85";

    #[tokio::test]
    async fn get_page_property_item() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = std::sync::Arc::new(notionrs::Client::new(notion_api_key));

        let request = client
            .get_page::<std::collections::HashMap<String, notionrs_types::prelude::PageProperty>>()
            .page_id(PAGE_ID);

        let response = request.send().await?;

        let properties = response
            .properties
            .into_iter()
            .map(|item| {
                let property_name = item.0;
                let property_id = match item.1 {
                    notionrs_types::prelude::PageProperty::Button(page_button_property) => {
                        page_button_property.id
                    }
                    notionrs_types::prelude::PageProperty::Checkbox(page_checkbox_property) => {
                        page_checkbox_property.id
                    }
                    notionrs_types::prelude::PageProperty::CreatedBy(page_created_by_property) => {
                        page_created_by_property.id
                    }
                    notionrs_types::prelude::PageProperty::CreatedTime(
                        page_created_time_property,
                    ) => page_created_time_property.id,
                    notionrs_types::prelude::PageProperty::Date(page_date_property) => {
                        page_date_property.id
                    }
                    notionrs_types::prelude::PageProperty::Email(page_email_property) => {
                        page_email_property.id
                    }
                    notionrs_types::prelude::PageProperty::Files(page_files_property) => {
                        page_files_property.id
                    }
                    notionrs_types::prelude::PageProperty::Formula(page_formula_property) => {
                        page_formula_property.id
                    }
                    notionrs_types::prelude::PageProperty::LastEditedBy(
                        page_last_edited_by_property,
                    ) => page_last_edited_by_property.id,
                    notionrs_types::prelude::PageProperty::LastEditedTime(
                        page_last_edited_time_property,
                    ) => page_last_edited_time_property.id,
                    notionrs_types::prelude::PageProperty::MultiSelect(
                        page_multi_select_property,
                    ) => page_multi_select_property.id,
                    notionrs_types::prelude::PageProperty::Number(page_number_property) => {
                        page_number_property.id
                    }
                    notionrs_types::prelude::PageProperty::People(page_people_property) => {
                        page_people_property.id
                    }
                    notionrs_types::prelude::PageProperty::PhoneNumber(
                        page_phone_number_property,
                    ) => page_phone_number_property.id,
                    notionrs_types::prelude::PageProperty::Place(page_place_property) => {
                        page_place_property.id
                    }
                    notionrs_types::prelude::PageProperty::Relation(page_relation_property) => {
                        page_relation_property.id
                    }
                    notionrs_types::prelude::PageProperty::RichText(page_rich_text_property) => {
                        page_rich_text_property.id
                    }
                    notionrs_types::prelude::PageProperty::Rollup(page_rollup_property) => {
                        page_rollup_property.id
                    }
                    notionrs_types::prelude::PageProperty::Select(page_select_property) => {
                        page_select_property.id
                    }
                    notionrs_types::prelude::PageProperty::Status(page_status_property) => {
                        page_status_property.id
                    }
                    notionrs_types::prelude::PageProperty::Title(page_title_property) => {
                        page_title_property.id
                    }
                    notionrs_types::prelude::PageProperty::UniqueId(page_unique_id_property) => {
                        page_unique_id_property.id
                    }
                    notionrs_types::prelude::PageProperty::Url(page_url_property) => {
                        page_url_property.id
                    }
                    notionrs_types::prelude::PageProperty::Verification(
                        page_verification_property,
                    ) => page_verification_property.id,
                };
                (property_name, property_id.unwrap())
            })
            .collect::<Vec<_>>();

        let futures = properties.into_iter().map(|(property_name, property_id)| {
            let client = client.clone();
            let page_id = PAGE_ID.to_string();
            async move {
                let request = client
                    .get_page_property_item()
                    .page_id(&page_id)
                    .property_id(&property_name);

                let response = request.send().await?;

                println!(
                    "Property Name: {}, Property ID: {}",
                    property_name, property_id,
                );

                println!(" -> Response: {}", serde_json::to_string(&response)?,);

                Ok::<_, notionrs::Error>((property_name, response))
            }
        });

        futures::future::join_all(futures).await;

        Ok(())
    }
}
