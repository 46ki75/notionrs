use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/update-a-view>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct UpdateViewClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the view to update.
    pub(crate) view_id: Option<String>,

    /// The name of the view.
    pub(crate) name: Option<String>,

    /// Filter to apply to the view.
    pub(crate) filter: Option<serde_json::Value>,

    /// Sorts to apply to the view.
    pub(crate) sorts: Option<Vec<serde_json::Value>>,

    /// Quick filters to pin in the view's filter bar.
    pub(crate) quick_filters: Option<serde_json::Value>,

    /// View presentation configuration.
    pub(crate) configuration: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateViewRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sorts: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quick_filters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<serde_json::Value>,
}

impl UpdateViewClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::view::ViewResponse, crate::error::Error> {
        let view_id = self.view_id.ok_or(crate::error::Error::RequestParameter(
            "`view_id` is not set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/views/{}", view_id);

        let req = UpdateViewRequestBody {
            name: self.name,
            filter: self.filter,
            sorts: self.sorts,
            quick_filters: self.quick_filters,
            configuration: self.configuration,
        };

        let request_body = serde_json::to_string(&req)?;

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

        let view = serde_json::from_slice::<notionrs_types::object::view::ViewResponse>(&body)?;

        Ok(view)
    }
}
