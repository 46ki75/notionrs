use serde::{Deserialize, Serialize};

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct SearchDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) query: Option<String>,

    pub(crate) sort: Option<crate::search::SearchSort>,

    /// Specify the cursor position at the beginning. In the query result,
    /// there is a field called `next_cursor` through
    /// which information is passed at the end.
    pub(crate) start_cursor: Option<String>,

    /// The amount of data retrieved in one query.
    /// If not specified, the default is 100.
    pub(crate) page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchDatabaseRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<crate::search::SearchFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sort: Option<crate::search::SearchSort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl SearchDatabaseClient {
    pub async fn send(
        self,
    ) -> Result<
        crate::list_response::ListResponse<crate::database::DatabaseResponse>,
        crate::error::Error,
    > {
        let url = String::from("https://api.notion.com/v1/search");

        let request_body = serde_json::to_string(&SearchDatabaseRequestBody {
            query: self.query,
            filter: Some(crate::search::SearchFilter::database()),
            sort: self.sort,
            start_cursor: self.start_cursor,
            page_size: self.page_size,
        })?;

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

        let pages = serde_json::from_slice::<
            crate::list_response::ListResponse<crate::database::DatabaseResponse>,
        >(&body)?;

        Ok(pages)
    }
}
