use serde::{Deserialize, Serialize};

use crate::RichText;

#[derive(Debug, Default)]
pub struct UpdateDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,

    pub(crate) title: Vec<RichText>,

    /// Field that can be added, though not documented in Notion's API documentation.
    /// Can be used as a description for the database.
    pub(crate) description: Vec<RichText>,

    /// When updating, passing a value of `null` (None) will remove the property.
    /// Note that it differs from the `create_database()` method in that it is optional.
    pub(crate) properties:
        std::collections::HashMap<String, Option<crate::database::DatabaseProperty>>,

    /// This can be configured even though it's not in the official Notion API documentation
    pub(crate) icon: Option<crate::others::icon::Icon>,

    /// This can be configured even though it's not in the official Notion API documentation
    pub(crate) cover: Option<crate::File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDatabaseRequestBody {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) title: Vec<RichText>,

    /// Field that can be added, though not documented in Notion's API documentation.
    /// Can be used as a description for the database.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) description: Vec<RichText>,

    /// When updating, passing a value of `null` (None) will remove the property.
    /// Note that it differs from the `create_database()` method in that it is optional.
    pub(crate) properties:
        std::collections::HashMap<String, Option<crate::database::DatabaseProperty>>,

    /// This can be configured even though it's not in the official Notion API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<crate::others::icon::Icon>,

    /// This can be configured even though it's not in the official Notion API documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<crate::File>,
}

impl UpdateDatabaseClient {
    pub async fn send(self) -> Result<crate::database::DatabaseResponse, crate::error::Error> {
        let database_id = self
            .database_id
            .ok_or(crate::error::Error::RequestParameter(
                "The database_id parameter must be defined.".to_string(),
            ))?;

        let request_body_struct = UpdateDatabaseRequestBody {
            properties: self.properties,
            title: self.title,
            description: self.description,
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

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::try_from_response_async(response).await);
        }

        let body = response.bytes().await?;

        let database: crate::database::DatabaseResponse =
            serde_json::from_slice::<crate::database::DatabaseResponse>(&body)?;

        Ok(database)
    }

    pub fn databse_id<T: AsRef<str>>(mut self, databse_id: T) -> Self {
        self.database_id = Some(databse_id.as_ref().to_string());
        self
    }

    pub fn title(mut self, title: Vec<RichText>) -> Self {
        self.title = title;
        self
    }

    /// Field that can be added, though not documented in Notion's API documentation.
    /// Can be used as a description for the database.
    pub fn description(mut self, title: Vec<RichText>) -> Self {
        self.description = title;
        self
    }

    /// When updating, passing a value of `null` (None) will remove the property.
    /// Note that it differs from the `create_database()` method in that it is optional.
    pub fn properties(
        mut self,
        properties: std::collections::HashMap<String, Option<crate::database::DatabaseProperty>>,
    ) -> Self {
        self.properties = properties;
        self
    }

    pub fn icon(mut self, icon: crate::others::icon::Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn cover(mut self, cover: crate::File) -> Self {
        self.cover = Some(cover);
        self
    }
}
