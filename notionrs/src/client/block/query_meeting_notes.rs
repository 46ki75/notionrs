use serde::{Deserialize, Serialize};

use notionrs_types::object::request::meeting_notes::{
    MeetingNotesCombinatorFilter, MeetingNotesSort,
};

/// Client for the query meeting notes endpoint.
///
/// <https://developers.notion.com/reference/query-meeting-notes>
#[derive(Debug, Default)]
pub struct QueryMeetingNotesClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) filter: Option<MeetingNotesCombinatorFilter>,

    pub(crate) sort: Vec<MeetingNotesSort>,

    pub(crate) limit: Option<u32>,
}

impl QueryMeetingNotesClient {
    /// Set the filter for the meeting notes query.
    pub fn filter(mut self, filter: MeetingNotesCombinatorFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Set the sort order for the meeting notes query.
    pub fn sort(mut self, sort: Vec<MeetingNotesSort>) -> Self {
        self.sort = sort;
        self
    }

    /// Set the maximum number of results to return. Defaults to 50.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryMeetingNotesRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<MeetingNotesCombinatorFilter>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) sort: Vec<MeetingNotesSort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit: Option<u32>,
}

impl QueryMeetingNotesClient {
    /// Send the query meeting notes request.
    ///
    /// <https://developers.notion.com/reference/query-meeting-notes>
    pub async fn send(
        self,
    ) -> Result<
        notionrs_types::object::block::QueryMeetingNotesResponse,
        crate::error::Error,
    > {
        let url = "https://api.notion.com/v1/blocks/meeting_notes/query";

        let request_body = serde_json::to_string(&QueryMeetingNotesRequestBody {
            filter: self.filter,
            sort: self.sort,
            limit: self.limit,
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

        let result = serde_json::from_slice::<
            notionrs_types::object::block::QueryMeetingNotesResponse,
        >(&body)?;

        Ok(result)
    }
}
