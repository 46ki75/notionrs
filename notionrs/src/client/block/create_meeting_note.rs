use notionrs_types::object::request::meeting_notes::{
    CreateMeetingNoteLanguage, CreateMeetingNoteSource,
};
use serde::Serialize;

/// Client for the create meeting note endpoint.
///
/// A file upload source requires `parent_page_id`. An existing block source
/// must omit it.
///
/// <https://developers.notion.com/reference/create-meeting-note>
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct CreateMeetingNoteClient {
    /// The reqwest HTTP client.
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) source: Option<CreateMeetingNoteSource>,

    pub(crate) parent_page_id: Option<String>,

    pub(crate) title: Option<String>,

    pub(crate) language: Option<CreateMeetingNoteLanguage>,

    pub(crate) kickoff_summary: Option<bool>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct CreateMeetingNoteRequestBody {
    source: CreateMeetingNoteSource,

    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<CreateMeetingNoteParent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<CreateMeetingNoteLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<CreateMeetingNoteOptions>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct CreateMeetingNoteParent {
    r#type: &'static str,
    page_id: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct CreateMeetingNoteOptions {
    kickoff_summary: bool,
}

impl CreateMeetingNoteClient {
    /// Use a completed public API file upload as the source.
    pub fn file_upload_source(mut self, file_upload_id: impl AsRef<str>) -> Self {
        self.source = Some(CreateMeetingNoteSource::file_upload(file_upload_id));
        self
    }

    /// Use an existing audio, video, or file block as the source.
    pub fn block_source(mut self, block_id: impl AsRef<str>) -> Self {
        self.source = Some(CreateMeetingNoteSource::block(block_id));
        self
    }

    fn into_request_body(self) -> Result<CreateMeetingNoteRequestBody, crate::error::Error> {
        let source = self.source.ok_or(crate::error::Error::RequestParameter(
            "`source` is not set.".to_owned(),
        ))?;

        let parent = match (&source, self.parent_page_id) {
            (CreateMeetingNoteSource::FileUpload { .. }, Some(page_id)) => {
                Some(CreateMeetingNoteParent {
                    r#type: "page_id",
                    page_id,
                })
            }
            (CreateMeetingNoteSource::FileUpload { .. }, None) => {
                return Err(crate::error::Error::RequestParameter(
                    "`parent_page_id` is required for a file upload source.".to_owned(),
                ));
            }
            (CreateMeetingNoteSource::Block { .. }, Some(_)) => {
                return Err(crate::error::Error::RequestParameter(
                    "`parent_page_id` must not be set for a block source.".to_owned(),
                ));
            }
            (CreateMeetingNoteSource::Block { .. }, None) => None,
        };

        Ok(CreateMeetingNoteRequestBody {
            source,
            parent,
            title: self.title,
            language: self.language,
            options: self
                .kickoff_summary
                .map(|kickoff_summary| CreateMeetingNoteOptions { kickoff_summary }),
        })
    }

    /// Create a meeting note and begin processing its source media.
    ///
    /// <https://developers.notion.com/reference/create-meeting-note>
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::block::CreateMeetingNoteResponse, crate::error::Error> {
        let reqwest_client = self.reqwest_client.clone();
        let request_body = serde_json::to_string(&self.into_request_body()?)?;
        let request = reqwest_client
            .post("https://api.notion.com/v1/blocks/meeting_notes")
            .header("Content-Type", "application/json")
            .body(request_body);

        crate::util::send_and_convert(request).await
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn request_error(client: CreateMeetingNoteClient) -> String {
        match client.into_request_body().unwrap_err() {
            crate::error::Error::RequestParameter(message) => message,
            error => panic!("expected request parameter error, got {error:?}"),
        }
    }

    #[test]
    fn serialize_file_upload_request() {
        let body = CreateMeetingNoteClient::default()
            .file_upload_source("upload-id")
            .parent_page_id("page-id")
            .title("Weekly sync")
            .language(CreateMeetingNoteLanguage::English)
            .kickoff_summary(true)
            .into_request_body()
            .unwrap();

        assert_eq!(
            serde_json::to_value(body).unwrap(),
            serde_json::json!({
                "source": {
                    "type": "file_upload",
                    "file_upload_id": "upload-id"
                },
                "parent": {
                    "type": "page_id",
                    "page_id": "page-id"
                },
                "title": "Weekly sync",
                "language": "en",
                "options": {
                    "kickoff_summary": true
                }
            })
        );
    }

    #[test]
    fn serialize_block_request_without_parent() {
        let body = CreateMeetingNoteClient::default()
            .block_source("block-id")
            .into_request_body()
            .unwrap();

        assert_eq!(
            serde_json::to_value(body).unwrap(),
            serde_json::json!({
                "source": {
                    "type": "block",
                    "block_id": "block-id"
                }
            })
        );
    }

    #[test]
    fn validate_source_and_parent_requirements() {
        assert_eq!(
            request_error(CreateMeetingNoteClient::default()),
            "`source` is not set."
        );
        assert_eq!(
            request_error(CreateMeetingNoteClient::default().file_upload_source("upload-id")),
            "`parent_page_id` is required for a file upload source."
        );
        assert_eq!(
            request_error(
                CreateMeetingNoteClient::default()
                    .block_source("block-id")
                    .parent_page_id("page-id")
            ),
            "`parent_page_id` must not be set for a block source."
        );
    }
}
