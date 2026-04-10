use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/create-a-view>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct CreateViewClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the data source this view should be scoped to.
    pub(crate) data_source_id: Option<String>,

    /// The name of the view.
    pub(crate) name: Option<String>,

    /// The type of view to create.
    pub(crate) view_type: Option<notionrs_types::object::view::ViewType>,

    /// The ID of the database to create a view in.
    /// Mutually exclusive with `view_id` and `create_database`.
    pub(crate) database_id: Option<String>,

    /// The ID of a dashboard view to add this view to as a widget.
    /// Mutually exclusive with `database_id` and `create_database`.
    pub(crate) view_id: Option<String>,

    /// Filter to apply to the view.
    pub(crate) filter: Option<serde_json::Value>,

    /// Sorts to apply to the view.
    pub(crate) sorts: Option<Vec<serde_json::Value>>,

    /// View presentation configuration.
    pub(crate) configuration: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateViewRequestBody {
    data_source_id: String,
    name: String,
    r#type: notionrs_types::object::view::ViewType,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sorts: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<serde_json::Value>,
}

impl CreateViewClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::view::ViewResponse, crate::error::Error> {
        let data_source_id = self
            .data_source_id
            .ok_or(crate::error::Error::RequestParameter(
                "`data_source_id` is not set.".to_string(),
            ))?;

        let name = self.name.ok_or(crate::error::Error::RequestParameter(
            "`name` is not set.".to_string(),
        ))?;

        let view_type = self
            .view_type
            .ok_or(crate::error::Error::RequestParameter(
                "`view_type` is not set.".to_string(),
            ))?;

        let url = "https://api.notion.com/v1/views";

        let req = CreateViewRequestBody {
            data_source_id,
            name,
            r#type: view_type,
            database_id: self.database_id,
            view_id: self.view_id,
            filter: self.filter,
            sorts: self.sorts,
            configuration: self.configuration,
        };

        let request_body = serde_json::to_string(&req)?;

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

        let view = serde_json::from_slice::<notionrs_types::object::view::ViewResponse>(&body)?;

        Ok(view)
    }
}
