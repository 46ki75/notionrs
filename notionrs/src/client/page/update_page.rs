use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, notionrs_macro::Setter)]
pub struct UpdatePageClient<
    T = std::collections::HashMap<String, notionrs_types::object::page::PageProperty>,
> {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,

    pub(crate) properties: T,

    pub(crate) in_trash: Option<bool>,

    pub(crate) icon: Option<notionrs_types::object::emoji_and_icon::EmojiAndIcon>,

    pub(crate) cover: Option<notionrs_types::object::file::File>,

    pub(crate) template: Option<UpdatePageTemplate>,
}

impl<T> Default for UpdatePageClient<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            reqwest_client: reqwest::Client::default(),
            page_id: None,
            properties: Default::default(),
            in_trash: None,
            icon: None,
            cover: None,
            template: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePageRequestBody<T> {
    pub(crate) properties: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) in_trash: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<notionrs_types::object::emoji_and_icon::EmojiAndIcon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<notionrs_types::object::file::File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) template: Option<UpdatePageTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePageTemplate {
    /// Whether to apply the parent data source's default template (`"default"`)
    /// or a specific template by ID (`template_id`).
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

impl<T> UpdatePageClient<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    /// When you want to update a page from a specific template, use this method to set the template ID.
    pub fn template_id(mut self, template_id: String) -> Self {
        self.template = Some(UpdatePageTemplate {
            r#type: "template_id".to_string(),
            template_id: Some(template_id),
            timezone: None,
        });
        self
    }

    /// When you want to update a page from a specific template with a timezone, use this method.
    ///
    /// The `timezone` parameter is an IANA timezone string (e.g. `"America/New_York"`) used
    /// when resolving template variables like @now and @today.
    pub fn template_id_with_timezone(mut self, template_id: String, timezone: String) -> Self {
        self.template = Some(UpdatePageTemplate {
            r#type: "template_id".to_string(),
            template_id: Some(template_id),
            timezone: Some(timezone),
        });
        self
    }

    /// When you want to update a page from the default template of the parent data source, use this method.
    pub fn template_default(mut self) -> Self {
        self.template = Some(UpdatePageTemplate {
            r#type: "default".to_string(),
            template_id: None,
            timezone: None,
        });
        self
    }

    /// When you want to update a page from the default template with a timezone, use this method.
    ///
    /// The `timezone` parameter is an IANA timezone string (e.g. `"America/New_York"`) used
    /// when resolving template variables like @now and @today.
    pub fn template_default_with_timezone(mut self, timezone: String) -> Self {
        self.template = Some(UpdatePageTemplate {
            r#type: "default".to_string(),
            template_id: None,
            timezone: Some(timezone),
        });
        self
    }

    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page::PageResponse<T>, crate::error::Error> {
        let page_id = self.page_id.ok_or(crate::error::Error::RequestParameter(
            "`page_id` is not set.".to_string(),
        ))?;

        let request_body_struct = UpdatePageRequestBody {
            properties: self.properties,
            in_trash: self.in_trash,
            icon: self.icon,
            cover: self.cover,
            template: self.template,
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

        let page = serde_json::from_slice::<notionrs_types::object::page::PageResponse<T>>(&body)?;

        Ok(page)
    }
}
