use notionrs_types::prelude::*;
use serde::{Deserialize, Serialize};

/// @see <https://developers.notion.com/reference/create-a-data-source>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct CreateDataSourceClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the parent database (with or without dashes),
    pub(crate) database_id: Option<String>,

    /// Property schema of the new data source.
    pub(crate) properties: Option<DataSourceProperty>,

    /// Property schema of the new data source.
    pub(crate) title: Vec<RichText>,

    /// Icon to apply to the data source.
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataSourceRequestBody {
    pub(crate) parent: DatabaseParent,
    pub(crate) properties: DataSourceProperty,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) title: Vec<RichText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,
}

impl CreateDataSourceClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::data_source::DataSourceResponse, crate::error::Error> {
        let url = "https://api.notion.com/v1/data_sources";

        let request_body =
            if let (Some(database_id), Some(properties)) = (self.database_id, self.properties) {
                let req = CreateDataSourceRequestBody {
                    parent: DatabaseParent {
                        r#type: "database".to_owned(),
                        database_id,
                    },
                    properties,
                    title: self.title,
                    icon: self.icon,
                };

                let string = serde_json::to_string(&req)?;

                Ok(string)
            } else {
                Err(crate::error::Error::RequestParameter(
                    "Either parent.database_id or properties is not specified.".to_string(),
                ))
            }?;

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

        let data_source = serde_json::from_slice::<
            notionrs_types::object::data_source::DataSourceResponse,
        >(&body)?;

        Ok(data_source)
    }
}
