use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/list-views>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ListViewsClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// ID of a Notion database to list views for.
    /// At least one of `database_id` or `data_source_id` is required.
    pub(crate) database_id: Option<String>,

    /// ID of a data source to list all views for.
    /// At least one of `database_id` or `data_source_id` is required.
    pub(crate) data_source_id: Option<String>,

    /// Cursor for pagination.
    pub(crate) start_cursor: Option<String>,

    /// The number of items from the full list desired in the response. Maximum: 100.
    pub(crate) page_size: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ListViewsRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    database_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u8>,
}

impl ListViewsClient {
    pub async fn send(
        self,
    ) -> Result<
        notionrs_types::object::response::ListResponse<
            notionrs_types::object::view::ViewReference,
        >,
        crate::error::Error,
    > {
        if self.database_id.is_none() && self.data_source_id.is_none() {
            return Err(crate::error::Error::RequestParameter(
                "At least one of `database_id` or `data_source_id` must be set.".to_string(),
            ));
        }

        let request_params = serde_urlencoded::to_string(&ListViewsRequestParams {
            database_id: self.database_id,
            data_source_id: self.data_source_id,
            start_cursor: self.start_cursor,
            page_size: self.page_size,
        })?;

        let url = format!("https://api.notion.com/v1/views?{}", request_params);

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

        let views = serde_json::from_slice::<
            notionrs_types::object::response::ListResponse<
                notionrs_types::object::view::ViewReference,
            >,
        >(&body)?;

        Ok(views)
    }
}
