use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/create-a-view-query>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct CreateViewQueryClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the view to query.
    pub(crate) view_id: Option<String>,

    /// The number of results to return per page. Maximum: 100.
    pub(crate) page_size: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateViewQueryRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u8>,
}

impl CreateViewQueryClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::view::ViewQueryResponse, crate::error::Error> {
        let view_id = self.view_id.ok_or(crate::error::Error::RequestParameter(
            "`view_id` is not set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/views/{}/queries", view_id);

        let req = CreateViewQueryRequestBody {
            page_size: self.page_size,
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

        let query =
            serde_json::from_slice::<notionrs_types::object::view::ViewQueryResponse>(&body)?;

        Ok(query)
    }
}
