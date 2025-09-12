use serde::{Deserialize, Serialize};

use notionrs_types::object::rich_text::RichText;

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct CreateDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,

    pub(crate) title: Vec<RichText>,

    /// Field that can be added, though not documented in Notion's API documentation.
    /// Can be used as a description for the database.
    pub(crate) description: Vec<RichText>,

    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::data_source::DataSourceProperty>,

    /// This can be configured even though it's not in the official Notion API documentation
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    /// This can be configured even though it's not in the official Notion API documentation
    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDatabaseRequestBodyPropertyPart {
    pub(crate) initial_data_source:
        std::collections::HashMap<String, notionrs_types::object::data_source::DataSourceProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDatabaseRequestBody {
    pub(crate) parent: notionrs_types::object::parent::PageParent,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) title: Vec<RichText>,

    /// Field that can be added, though not documented in Notion's API documentation.
    /// Can be used as a description for the database.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) description: Vec<RichText>,

    pub(crate) properties: CreateDatabaseRequestBodyPropertyPart,

    /// This can be configured even though it's not in the official Notion API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    /// This can be configured even though it's not in the official Notion API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

impl CreateDatabaseClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::data_source::DatabaseResponse, crate::error::Error> {
        let page_id = self.page_id.unwrap();

        let request_body_struct = CreateDatabaseRequestBody {
            parent: notionrs_types::object::parent::PageParent::from(page_id),
            properties: CreateDatabaseRequestBodyPropertyPart {
                initial_data_source: self.properties,
            },
            title: self.title,
            description: self.description,
            icon: self.icon,
            cover: self.cover,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = "https://api.notion.com/v1/databases".to_string();

        let request = self
            .reqwest_client
            .post(url)
            .header("Content-Type", "application/json")
            .body(request_body);

        let response = request
            .send()
            .await
            .map_err(|e| crate::error::Error::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(crate::error::Error::try_from_response_async(response).await);
        }

        let body = response
            .bytes()
            .await
            .map_err(|e| crate::error::Error::BodyParse(e.to_string()))?;

        let database: notionrs_types::object::data_source::DatabaseResponse =
            serde_json::from_slice::<notionrs_types::object::data_source::DatabaseResponse>(&body)?;

        Ok(database)
    }
}
