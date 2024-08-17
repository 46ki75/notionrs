use crate::error::{api_error::NotionApiError, NotionError};

#[derive(Debug)]
pub struct GetPagePropertyItemClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,

    pub(crate) property_id: Option<String>,
}

impl GetPagePropertyItemClient {
    // TODO: docs for send
    pub async fn send(self) -> Result<crate::page::properties::PageProperty, NotionError> {
        let page_id = self
            .page_id
            .ok_or(NotionError::NotionRequestParameterError(
                "`page_id` has not been set.".to_string(),
            ))?;

        let property_id = self
            .property_id
            .ok_or(NotionError::NotionRequestParameterError(
                "`property_id` has not been set.".to_string(),
            ))?;

        let url = format!(
            "https://api.notion.com/v1/pages/{}/properties/{}",
            page_id, property_id
        );

        let request = self.reqwest_client.get(url);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.text().await?;

            let error_json = serde_json::from_str::<NotionApiError>(&error_body)?;

            return Err(NotionError::NotionApiError(Box::new(error_json)));
        }

        let body = response.text().await?;

        let user = serde_json::from_str::<crate::page::properties::PageProperty>(&body)?;

        Ok(user)
    }

    // TODO: docs for page_id
    pub fn page_id<T: AsRef<str>>(mut self, page_id: T) -> Self {
        self.page_id = Some(page_id.as_ref().to_string());
        self
    }

    // TODO: docs for property_id
    pub fn property_id<T: AsRef<str>>(mut self, property_id: T) -> Self {
        self.property_id = Some(property_id.as_ref().to_string());
        self
    }
}
