use serde::{Deserialize, Serialize};

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct UpdatePageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,

    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,

    pub(crate) in_trash: Option<bool>,

    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePageRequestBody {
    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) in_trash: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

impl UpdatePageClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page::PageResponse, crate::error::Error> {
        let page_id = self.page_id.ok_or(crate::error::Error::RequestParameter(
            "`page_id` is not set.".to_string(),
        ))?;

        let request_body_struct = UpdatePageRequestBody {
            properties: self.properties,
            in_trash: self.in_trash,
            icon: self.icon,
            cover: self.cover,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = format!("https://api.notion.com/v1/pages/{}", page_id);

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

        let page = serde_json::from_slice::<notionrs_types::object::page::PageResponse>(&body)?;

        Ok(page)
    }
}
