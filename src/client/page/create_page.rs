use serde::{Deserialize, Serialize};

use crate::page::page_response::PageResponse;

#[derive(Debug, Default)]
pub struct CreatePageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Cannot specify the same page ID as the parent page's database_id  
    pub(crate) page_id: Option<String>,

    /// Cannot specify the same database ID as the parent database's page_id  
    pub(crate) database_id: Option<String>,

    pub(crate) properties: std::collections::HashMap<String, crate::page::properties::PageProperty>,

    pub(crate) children: Option<Vec<crate::block::Block>>,

    pub(crate) icon: Option<crate::others::icon::Icon>,

    pub(crate) cover: Option<crate::File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePageRequestBody {
    pub(crate) parent: crate::others::parent::Parent,

    pub(crate) properties: std::collections::HashMap<String, crate::page::properties::PageProperty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) children: Option<Vec<crate::block::Block>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<crate::others::icon::Icon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<crate::File>,
}

impl CreatePageClient {
    /// Send a request specifying generics.
    ///
    /// Create a struct with generics like `send::<MyResponse>()`.
    /// When the response type is not specific,
    /// use `send::<HashMap<String, PageProperty>>()`.
    /// (Type inference for the property field cannot be used.)
    pub async fn send(self) -> Result<PageResponse, crate::error::Error> {
        let mut parent: Option<crate::others::parent::Parent> = None;

        if let Some(page_id) = self.page_id {
            parent = Some(crate::others::parent::Parent::PageParent(
                crate::others::parent::PageParent::from(page_id),
            ));
        }

        if let Some(database_id) = self.database_id {
            parent = Some(crate::others::parent::Parent::DatabaseParent(
                crate::others::parent::DatabaseParent::from(database_id),
            ));
        }

        let parent = parent.ok_or_else(|| {
            crate::error::Error::RequestParameter(
                "You need to specify either the page_id or the database_id.".to_string(),
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

        let page: PageResponse = serde_json::from_slice::<PageResponse>(&body)?;

        Ok(page)
    }

    pub fn page_id<T: AsRef<str>>(mut self, page_id: T) -> Self {
        self.page_id = Some(page_id.as_ref().to_string());
        self
    }

    pub fn database_id<T: AsRef<str>>(mut self, database_id: T) -> Self {
        self.database_id = Some(database_id.as_ref().to_string());
        self
    }

    pub fn properties(
        mut self,
        properties: std::collections::HashMap<String, crate::page::properties::PageProperty>,
    ) -> Self {
        self.properties = properties;
        self
    }

    pub fn children(mut self, children: Vec<crate::block::Block>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn icon(mut self, icon: crate::others::icon::Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn cover(mut self, cover: crate::File) -> Self {
        self.cover = Some(cover);
        self
    }
}
