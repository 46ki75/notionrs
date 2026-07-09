use std::marker::PhantomData;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// @see <https://developers.notion.com/reference/post-page>
#[derive(Debug, notionrs_macro::Setter)]
pub struct CreatePageClient<
    T = std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,
> {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Cannot specify the same page ID as the parent page's data_source_id  
    pub(crate) page_id: Option<String>,

    /// Cannot specify the same data_source ID as the parent data_source's page_id  
    pub(crate) data_source_id: Option<String>,

    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,

    pub(crate) children: Option<Vec<notionrs_types::object::block::Block>>,

    /// Page content as Notion-flavored Markdown. Mutually exclusive with `children`.
    pub(crate) markdown: Option<String>,

    pub(crate) icon: Option<notionrs_types::object::emoji_and_icon::EmojiAndIcon>,

    pub(crate) cover: Option<notionrs_types::object::file::File>,

    pub(crate) template: Option<CreatePageTemplate>,

    /// Customizing position within a parent page
    ///
    /// When the parent is a page, a reference to the newly created page is automatically added to the bottom of the parent by default.
    ///
    /// To customize this behavior, use the optional position parameter (added in January 2026 as part of v5.7.0 of the JS SDK):
    ///
    /// - To insert the child_page mention after a specific block, pass `"position": {"type": "after_block", "after_block": {"id": "BLOCK_ID"}}`.
    /// - To create the page at the top, set `"position": {"type": "page_start"}`.
    /// - The default behavior (inserting at the bottom) is represented by `"position": {"type": "page_end"}`.
    ///
    /// **Note:** The position parameter is not allowed unless the parent is a page.
    pub(crate) position: Option<CreatePageTemplatePosition>,

    /// When set to `true` and `markdown` is provided, the page may be created
    /// asynchronously; the response can then be an async task reference instead
    /// of the created page. Added in `notion-sdk-js` v5.23.0.
    pub(crate) allow_async: Option<bool>,

    #[setter(skip)]
    pub(crate) _phantom: PhantomData<T>,
}

impl<T> Default for CreatePageClient<T> {
    fn default() -> Self {
        Self {
            reqwest_client: reqwest::Client::default(),
            page_id: None,
            data_source_id: None,
            properties: std::collections::HashMap::new(),
            children: None,
            markdown: None,
            icon: None,
            cover: None,
            template: None,
            position: None,
            allow_async: None,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePageRequestBody {
    pub(crate) parent: notionrs_types::object::parent::Parent,

    pub(crate) properties:
        std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) children: Option<Vec<notionrs_types::object::block::Block>>,

    /// Page content as Notion-flavored Markdown. Mutually exclusive with `children`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) markdown: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::emoji_and_icon::EmojiAndIcon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<notionrs_types::object::file::File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) template: Option<CreatePageTemplate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<CreatePageTemplatePosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allow_async: Option<bool>,
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

    /// IANA timezone to use when resolving template variables like @now and @today
    /// (e.g. `"America/New_York"`). Defaults to the authorizing user's timezone for
    /// public integrations, or UTC for internal integrations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timezone: Option<String>,
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

impl<T> CreatePageClient<T> {
    /// Change the page-property type used to deserialize the response.
    /// Call this when you want to map properties into a custom struct instead
    /// of the default `HashMap<String, PageProperty>`.
    pub fn typed<U>(self) -> CreatePageClient<U> {
        CreatePageClient {
            reqwest_client: self.reqwest_client,
            page_id: self.page_id,
            data_source_id: self.data_source_id,
            properties: self.properties,
            children: self.children,
            markdown: self.markdown,
            icon: self.icon,
            cover: self.cover,
            template: self.template,
            position: self.position,
            allow_async: self.allow_async,
            _phantom: PhantomData,
        }
    }

    /// When you want to create a page from a specific template, use this method to set the template ID.
    pub fn template_id(mut self, template_id: String) -> Self {
        self.template = Some(CreatePageTemplate {
            r#type: "template_id".to_string(),
            template_id: Some(template_id),
            timezone: None,
        });
        self
    }

    /// When you want to create a page from a specific template with a timezone, use this method.
    ///
    /// The `timezone` parameter is an IANA timezone string (e.g. `"America/New_York"`) used
    /// when resolving template variables like @now and @today.
    pub fn template_id_with_timezone(mut self, template_id: String, timezone: String) -> Self {
        self.template = Some(CreatePageTemplate {
            r#type: "template_id".to_string(),
            template_id: Some(template_id),
            timezone: Some(timezone),
        });
        self
    }

    /// When you want to create a page from the default template of the parent data source, use this method.
    pub fn template_default(mut self) -> Self {
        self.template = Some(CreatePageTemplate {
            r#type: "default".to_string(),
            template_id: None,
            timezone: None,
        });
        self
    }

    /// When you want to create a page from the default template with a timezone, use this method.
    ///
    /// The `timezone` parameter is an IANA timezone string (e.g. `"America/New_York"`) used
    /// when resolving template variables like @now and @today.
    pub fn template_default_with_timezone(mut self, timezone: String) -> Self {
        self.template = Some(CreatePageTemplate {
            r#type: "default".to_string(),
            template_id: None,
            timezone: Some(timezone),
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

    /// Send the request and deserialize the response properties into type `T`.
    ///
    /// Use `typed::<MyResponse>()` before calling `send()` to specify a custom struct.
    /// When the response type is not specific, the default `HashMap<String, PageProperty>` is used.
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page::PageResponse<T>, crate::error::Error>
    where
        T: DeserializeOwned + Clone + Send + Sync + 'static,
    {
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

        if self.children.is_some() && self.markdown.is_some() {
            return Err(crate::error::Error::RequestParameter(
                "`children` and `markdown` are mutually exclusive. Please set only one of them."
                    .to_string(),
            ));
        }

        let request_body_struct = CreatePageRequestBody {
            parent,
            properties: self.properties,
            children: self.children,
            markdown: self.markdown,
            icon: self.icon,
            cover: self.cover,
            template: self.template,
            position: self.position,
            allow_async: self.allow_async,
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

        let page = serde_json::from_slice::<notionrs_types::object::page::PageResponse<T>>(&body)?;

        Ok(page)
    }
}

// # --------------------------------------------------------------------------------
//
// unit_tests
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_page_request_body_with_markdown() {
        let request_body = CreatePageRequestBody {
            parent: notionrs_types::object::parent::Parent::PageParent(
                notionrs_types::object::parent::PageParent::from("page-id-123"),
            ),
            properties: std::collections::HashMap::new(),
            children: None,
            markdown: Some("# Hello World\n\nThis is a test.".to_string()),
            icon: None,
            cover: None,
            template: None,
            position: None,
            allow_async: None,
        };

        let json = serde_json::to_value(&request_body).expect("Failed to serialize");
        assert_eq!(json["markdown"], "# Hello World\n\nThis is a test.");
        assert!(json.get("children").is_none());
    }

    #[test]
    fn serialize_create_page_request_body_without_markdown() {
        let request_body = CreatePageRequestBody {
            parent: notionrs_types::object::parent::Parent::PageParent(
                notionrs_types::object::parent::PageParent::from("page-id-123"),
            ),
            properties: std::collections::HashMap::new(),
            children: None,
            markdown: None,
            icon: None,
            cover: None,
            template: None,
            position: None,
            allow_async: None,
        };

        let json = serde_json::to_value(&request_body).expect("Failed to serialize");
        assert!(json.get("markdown").is_none());
        assert!(json.get("children").is_none());
    }

    #[test]
    fn serialize_create_page_request_body_with_allow_async() {
        let request_body = CreatePageRequestBody {
            parent: notionrs_types::object::parent::Parent::PageParent(
                notionrs_types::object::parent::PageParent::from("page-id-123"),
            ),
            properties: std::collections::HashMap::new(),
            children: None,
            markdown: Some("# Hello World".to_string()),
            icon: None,
            cover: None,
            template: None,
            position: None,
            allow_async: Some(true),
        };

        let json = serde_json::to_value(&request_body).expect("Failed to serialize");
        assert_eq!(json["allow_async"], true);
    }

    #[test]
    fn create_page_client_allow_async_setter() {
        let client = CreatePageClient::<
            std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,
        >::default()
        .page_id("page-id-123")
        .markdown("# Hello World")
        .allow_async(true);

        assert_eq!(client.allow_async, Some(true));
    }

    #[test]
    fn create_page_client_markdown_setter() {
        let client = CreatePageClient::<
            std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,
        >::default()
        .page_id("page-id-123")
        .markdown("# Hello World");

        assert_eq!(client.markdown, Some("# Hello World".to_string()));
    }

    #[tokio::test]
    async fn create_page_client_rejects_children_and_markdown() {
        let client = CreatePageClient::<
            std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,
        >::default()
        .page_id("page-id-123")
        .children(vec![])
        .markdown("# Hello World");

        let result = client.send().await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            crate::error::Error::RequestParameter(msg) => {
                assert!(msg.contains("mutually exclusive"));
            }
            _ => panic!("Expected RequestParameter error, got: {:?}", err),
        }
    }
}
