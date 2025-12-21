use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/move-page>
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct MovePageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the page to move
    pub(crate) source_page_id: Option<String>,

    /// The ID of the destination page to move the page to
    pub(crate) destination_page_id: Option<String>,

    /// The ID of the destination data source to move the page to
    pub(crate) destination_data_source_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovePageRequestBody {
    pub(crate) parent: MovePageDestination,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MovePageDestination {
    #[serde(rename = "page_id")]
    PageId { page_id: String },

    #[serde(rename = "data_source_id")]
    DataSourceId { data_source_id: String },
}

impl MovePageClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page::PageResponse, crate::error::Error> {
        let source_page_id = self
            .source_page_id
            .ok_or(crate::error::Error::RequestParameter(
                "`source_page_id` is not set.".to_owned(),
            ))?;

        let parent =
            if let (None, None) = (&self.destination_page_id, &self.destination_data_source_id) {
                Err(crate::error::Error::RequestParameter(
                    "Either `destination_page_id` or `destination_data_source_id` must be set."
                        .to_owned(),
                ))
            } else if let (Some(_), Some(_)) =
                (&self.destination_page_id, &self.destination_data_source_id)
            {
                Err(crate::error::Error::RequestParameter(
                    "Only one of `destination_page_id` or `destination_data_source_id` can be set."
                        .to_owned(),
                ))
            } else if let Some(destination_page_id) = &self.destination_page_id {
                Ok(MovePageDestination::PageId {
                    page_id: destination_page_id.clone(),
                })
            } else if let Some(destination_data_source_id) = &self.destination_data_source_id {
                Ok(MovePageDestination::DataSourceId {
                    data_source_id: destination_data_source_id.clone(),
                })
            } else {
                unreachable!()
            }?;

        let request_url = format!(
            "https://api.notion.com/v1/pages/{page_id}/move",
            page_id = source_page_id
        );

        let request_body = MovePageRequestBody { parent };

        let request_body_string = serde_json::to_string(&request_body)?;

        let request = self
            .reqwest_client
            .post(request_url)
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

        let page = serde_json::from_slice::<notionrs_types::object::page::PageResponse>(&body)?;

        Ok(page)
    }
}

mod tests {

    #[test]
    fn deserialize_move_page_request_body() {
        let json_data = r#"
        {
            "parent": {
                "type": "page_id",
                "page_id": "some-page-id"
            }
        }
        "#;

        let deserialized: super::MovePageRequestBody =
            serde_json::from_str(json_data).expect("Failed to deserialize JSON");

        assert!(matches!(
            deserialized.parent,
            super::MovePageDestination::PageId { .. }
        ));

        match deserialized.parent {
            super::MovePageDestination::PageId { page_id } => {
                assert_eq!(page_id, "some-page-id");
            }
            _ => panic!("Expected MovePageDestination::PageId variant"),
        }
    }
}
