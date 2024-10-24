use serde::{Deserialize, Serialize};

use crate::{
    error::{api_error::ApiError, Error},
    RichText,
};

#[derive(Debug, Default)]
pub struct CreateDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,

    pub(crate) title: Vec<RichText>,

    pub(crate) properties: std::collections::HashMap<String, crate::database::DatabaseProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDatabaseRequestBody {
    pub(crate) parent: crate::others::parent::PageParent,

    pub(crate) title: Vec<RichText>,

    pub(crate) properties: std::collections::HashMap<String, crate::database::DatabaseProperty>,
}

impl CreateDatabaseClient {
    pub async fn send(self) -> Result<crate::database::DatabaseResponse, Error> {
        let page_id = self.page_id.unwrap();

        let request_body_struct = CreateDatabaseRequestBody {
            parent: crate::others::parent::PageParent::from(page_id),
            properties: self.properties,
            title: self.title,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = "https://api.notion.com/v1/databases".to_string();

        let request = self
            .reqwest_client
            .post(url)
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

    pub fn page_id<T: AsRef<str>>(mut self, page_id: T) -> Self {
        self.page_id = Some(page_id.as_ref().to_string());
        self
    }

    pub fn title(mut self, title: Vec<RichText>) -> Self {
        self.title = title;
        self
    }

    pub fn properties(
        mut self,
        properties: std::collections::HashMap<String, crate::database::DatabaseProperty>,
    ) -> Self {
        self.properties = properties;
        self
    }
}
