use serde::{Deserialize, Serialize};

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct CreatePageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Cannot specify the same page ID as the parent page's database_id  
    pub(crate) page_id: Option<String>,

    /// Cannot specify the same database ID as the parent database's page_id  
    pub(crate) database_id: Option<String>,

    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,

    pub(crate) children: Option<Vec<notionrs_types::object::block::Block>>,

    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePageRequestBody {
    pub(crate) parent: notionrs_types::object::parent::Parent,

    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) children: Option<Vec<notionrs_types::object::block::Block>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<notionrs_types::object::file::File>,
}

impl CreatePageClient {
    /// Send a request specifying generics.
    ///
    /// Create a struct with generics like `send::<MyResponse>()`.
    /// When the response type is not specific,
    /// use `send::<HashMap<String, PageProperty>>()`.
    /// (Type inference for the property field cannot be used.)
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page::PageResponse, crate::error::Error> {
        let mut parent: Option<notionrs_types::object::parent::Parent> = None;

        if let Some(page_id) = self.page_id {
            parent = Some(notionrs_types::object::parent::Parent::PageParent(
                notionrs_types::object::parent::PageParent::from(page_id),
            ));
        }

        if let Some(database_id) = self.database_id {
            parent = Some(notionrs_types::object::parent::Parent::DatabaseParent(
                notionrs_types::object::parent::DatabaseParent::from(database_id),
            ));
        }

        let parent = parent.ok_or_else(|| {
            crate::error::Error::RequestParameter(
                "Either `page_id` or `database_id` must be set.".to_string(),
            )
        })?;

        let request_body_struct = CreatePageRequestBody {
            parent,
            properties: self.properties,
            children: self.children,
            icon: self.icon,
            cover: self.cover,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = "https://api.notion.com/v1/pages".to_string();

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

        let page = serde_json::from_slice::<notionrs_types::object::page::PageResponse>(&body)?;

        Ok(page)
    }
}
