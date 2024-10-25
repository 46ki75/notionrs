use serde::{Deserialize, Serialize};

use crate::{
    error::{api_error::ApiError, Error},
    RichText,
};

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
}

impl UpdateDatabaseClient {
    pub async fn send(self) -> Result<crate::database::DatabaseResponse, Error> {
        let database_id = self
            .database_id
            .ok_or(crate::error::Error::RequestParameter(
                "The database_id parameter must be defined.".to_string(),
            ))?;

        let request_body_struct = UpdateDatabaseRequestBody {
            properties: self.properties,
            title: self.title,
            description: self.description,
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
            let error_body = response.text().await?;

            let error_json = serde_json::from_str::<ApiError>(&error_body)?;

            return Err(Error::Api(Box::new(error_json)));
        }

        let body = response.text().await?;

        let database: crate::database::DatabaseResponse =
            serde_json::from_str::<crate::database::DatabaseResponse>(&body)?;

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
}
