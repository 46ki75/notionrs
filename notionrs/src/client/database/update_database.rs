use serde::{Deserialize, Serialize};

use notionrs_types::object::rich_text::RichText;

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct UpdateDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,

    pub(crate) title: Vec<RichText>,

    /// Field that can be added, though not documented in Notion's API documentation.
    /// Can be used as a description for the database.
    pub(crate) description: Vec<RichText>,

    /// Whether the database should be displayed inline in the parent page.
    /// If not provided, the inline status will not be updated.
    pub(crate) is_inline: Option<bool>,

    /// When updating, passing a value of `null` (None) will remove the property.
    /// Note that it differs from the `create_database()` method in that it is optional.
    pub(crate) properties: std::collections::HashMap<
        String,
        Option<notionrs_types::object::database::DatabaseProperty>,
    >,

    /// This can be configured even though it's not in the official Notion API documentation
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    /// This can be configured even though it's not in the official Notion API documentation
    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDatabaseRequestBody {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) title: Vec<RichText>,

    /// Field that can be added, though not documented in Notion's API documentation.
    /// Can be used as a description for the database.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) description: Vec<RichText>,

    /// Whether the database should be displayed inline in the parent page.
    /// If not provided, the inline status will not be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_inline: Option<bool>,

    /// When updating, passing a value of `null` (None) will remove the property.
    /// Note that it differs from the `create_database()` method in that it is optional.
    pub(crate) properties: std::collections::HashMap<
        String,
        Option<notionrs_types::object::database::DatabaseProperty>,
    >,

    /// This can be configured even though it's not in the official Notion API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    /// This can be configured even though it's not in the official Notion API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

impl UpdateDatabaseClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::database::DatabaseResponse, crate::error::Error> {
        let database_id = self
            .database_id
            .ok_or(crate::error::Error::RequestParameter(
                "`database_id` is not set.".to_string(),
            ))?;

        let request_body_struct = UpdateDatabaseRequestBody {
            properties: self.properties,
            title: self.title,
            description: self.description,
            is_inline: self.is_inline,
            icon: self.icon,
            cover: self.cover,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = format!("https://api.notion.com/v1/databases/{}", database_id);

        let request = self
            .reqwest_client
            .patch(url)
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

        let database: notionrs_types::object::database::DatabaseResponse =
            serde_json::from_slice::<notionrs_types::object::database::DatabaseResponse>(&body)?;

        Ok(database)
    }
}
