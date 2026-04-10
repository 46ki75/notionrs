use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/get-view-query-results>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct GetViewQueryResultsClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the view.
    pub(crate) view_id: Option<String>,

    /// The ID of the query.
    pub(crate) query_id: Option<String>,

    /// Cursor for pagination.
    pub(crate) start_cursor: Option<String>,

    /// The number of results to return per page. Maximum: 100.
    pub(crate) page_size: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct GetViewQueryResultsRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u8>,
}

impl GetViewQueryResultsClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::view::ViewQueryResponse, crate::error::Error> {
        let view_id = self.view_id.ok_or(crate::error::Error::RequestParameter(
            "`view_id` is not set.".to_string(),
        ))?;

        let query_id = self.query_id.ok_or(crate::error::Error::RequestParameter(
            "`query_id` is not set.".to_string(),
        ))?;

        let request_params = serde_urlencoded::to_string(&GetViewQueryResultsRequestParams {
            start_cursor: self.start_cursor,
            page_size: self.page_size,
        })?;

        let url = format!(
            "https://api.notion.com/v1/views/{}/queries/{}?{}",
            view_id, query_id, request_params
        );

        let request = self.reqwest_client.get(url);

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

        let results =
            serde_json::from_slice::<notionrs_types::object::view::ViewQueryResponse>(&body)?;

        Ok(results)
    }
}
