use serde::{Deserialize, Serialize};

/// @see <https://developers.notion.com/reference/post-page>
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct CreatePageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Cannot specify the same page ID as the parent page's data_source_id  
    pub(crate) page_id: Option<String>,

    /// Cannot specify the same data_source ID as the parent data_source's page_id  
    pub(crate) data_source_id: Option<String>,

    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,

    pub(crate) children: Option<Vec<notionrs_types::object::block::Block>>,

    pub(crate) icon: Option<notionrs_types::object::icon::Icon>,

    pub(crate) cover: Option<notionrs_types::object::file::File>,

    pub(crate) template: Option<CreatePageTemplate>,

    pub(crate) position: Option<CreatePageTemplatePosition>,
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) template: Option<CreatePageTemplate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<CreatePageTemplatePosition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePageTemplate {
    /// Whether to apply no template and create a page from scratch manually (`"none"`),
    /// the parent data source's default template (`"default"`),
    /// or a specific template by ID (`template_id`). The default behavior is `"none"`.
    pub(crate) r#type: String,

    /// When `type=template_id`, provide the ID of a page template as the `template_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) template_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePageTemplatePosition {
    r#type: CreatePageTemplatePositionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_block: Option<CreatePageTemplatePositionAfterBlockPayload>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CreatePageTemplatePositionType {
    AfterBlock,
    PageStart,
    PageEnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePageTemplatePositionAfterBlockPayload {
    // Block ID
    id: String,
}

impl CreatePageClient {
    /// When you want to create a page from a specific template, use this method to set the template ID.
    pub fn template_id(mut self, template_id: String) -> Self {
        self.template = Some(CreatePageTemplate {
            r#type: "template_id".to_string(),
            template_id: Some(template_id),
        });
        self
    }

    /// When you want to create a page from the default template of the parent data source, use this method.
    pub fn template_default(mut self) -> Self {
        self.template = Some(CreatePageTemplate {
            r#type: "default".to_string(),
            template_id: None,
        });
        self
    }

    pub fn template_position_after_block(mut self, block_id: String) -> Self {
        self.position = Some(CreatePageTemplatePosition {
            r#type: CreatePageTemplatePositionType::AfterBlock,
            after_block: Some(CreatePageTemplatePositionAfterBlockPayload { id: block_id }),
        });
        self
    }

    pub fn template_position_page_start(mut self) -> Self {
        self.position = Some(CreatePageTemplatePosition {
            r#type: CreatePageTemplatePositionType::PageStart,
            after_block: None,
        });
        self
    }

    pub fn template_position_page_end(mut self) -> Self {
        self.position = Some(CreatePageTemplatePosition {
            r#type: CreatePageTemplatePositionType::PageEnd,
            after_block: None,
        });
        self
    }

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

        if let Some(data_source_id) = self.data_source_id {
            parent = Some(notionrs_types::object::parent::Parent::DataSourceParent(
                notionrs_types::object::parent::DataSourceParent::from(data_source_id),
            ));
        }

        let parent = parent.ok_or_else(|| {
            crate::error::Error::RequestParameter(
                "Either `page_id` or `data_source_id` must be set.".to_string(),
            )
        })?;

        let request_body_struct = CreatePageRequestBody {
            parent,
            properties: self.properties,
            children: self.children,
            icon: self.icon,
            cover: self.cover,
            template: self.template,
            position: self.position,
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
