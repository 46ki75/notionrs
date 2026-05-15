use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct DataSourceTemplate {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct DataSourceTemplateListResponse {
    pub templates: Vec<DataSourceTemplate>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_data_source_template_list_response() {
        let json_data = r#"
            {
                "templates": [
                    {
                        "id": "9f0c9dfb-ce14-4375-802f-339c1b8eecea",
                        "name": "My Template",
                        "is_default": false
                    }
                ],
                "has_more": false,
                "next_cursor": null
            }
        "#;

        let data_source_template_list_response =
            serde_json::from_str::<DataSourceTemplateListResponse>(&json_data).unwrap();

        assert!(data_source_template_list_response.templates.len() == 1);
        assert_eq!(
            data_source_template_list_response.templates[0].id,
            "9f0c9dfb-ce14-4375-802f-339c1b8eecea"
        );
        assert_eq!(
            data_source_template_list_response.templates[0].name,
            "My Template"
        );
        assert_eq!(
            data_source_template_list_response.templates[0].is_default,
            false
        );
        assert_eq!(data_source_template_list_response.has_more, false);
        assert_eq!(data_source_template_list_response.next_cursor, None);
    }

    #[test]
    fn template_property_setters() {
        let t = DataSourceTemplate::default()
            .id("id")
            .name("n")
            .is_default(true);
        assert_eq!(t.id, "id");
        assert_eq!(t.name, "n");
        assert!(t.is_default);

        let list = DataSourceTemplateListResponse::default()
            .templates(vec![t])
            .has_more(true)
            .next_cursor("cursor".to_string());
        assert!(list.has_more);
        assert_eq!(list.next_cursor.as_deref(), Some("cursor"));
    }
}
