use notionrs_types::prelude::*;
use serde::{Deserialize, Serialize};

/// @see <https://developers.notion.com/reference/create-a-data-source>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct UpdateDataSourceClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// ID of a Notion data source. This is a UUIDv4, with or without dashes.
    pub(crate) data_source_id: Option<String>,

    /// Property schema of the new data source.
    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::data_source::DataSourceProperty>,

    /// Property schema of the new data source.
    pub(crate) title: Vec<RichText>,

    /// Icon to apply to the data source.
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    /// Pass `true` to move a data source to the trash,
    /// or `false` to restore it from the trash.
    pub(crate) in_trash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDataSourceRequestBody {
    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::data_source::DataSourceProperty>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) title: Vec<RichText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) in_trash: Option<bool>,
}

impl UpdateDataSourceClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::data_source::DataSourceResponse, crate::error::Error> {
        let url = if let Some(data_source_id) = self.data_source_id {
            Ok(format!(
                "https://api.notion.com/v1/data_sources/{data_source_id}"
            ))
        } else {
            Err(crate::error::Error::RequestParameter(
                "data_source_id is not set.".to_string(),
            ))
        }?;

        let req = UpdateDataSourceRequestBody {
            properties: self.properties,
            title: self.title,
            icon: self.icon,
            in_trash: self.in_trash,
        };

        let request_body_string = serde_json::to_string(&req)?;

        let request = self
            .reqwest_client
            .patch(url)
            .header("Content-Type", "application/json")
            .body(request_body_string);

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
