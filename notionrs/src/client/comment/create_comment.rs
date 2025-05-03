use serde::Serialize;

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct CrateCommentClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,

    pub(crate) discussion_id: Option<String>,

    pub(crate) rich_text: Option<Vec<crate::object::rich_text::RichText>>,
}

#[derive(Debug, Serialize)]
pub struct CreateCommentRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<crate::object::parent::Parent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    discussion_id: Option<String>,

    rich_text: Vec<crate::object::rich_text::RichText>,
}

impl CrateCommentClient {
    pub async fn send(self) -> Result<crate::object::comment::Comment, crate::error::Error> {
        match self.rich_text {
            None => Err(crate::error::Error::RequestParameter(
                "rich_text is not set.".to_string(),
            )),
            Some(rich_text) => {
                if self.page_id.is_none() && self.discussion_id.is_none() {
                    Err(crate::error::Error::RequestParameter(
                        "either page_id or discussion_id must be provided".to_string(),
                    ))
                } else if self.page_id.is_some() && self.discussion_id.is_some() {
                    Err(crate::error::Error::RequestParameter(
                        "only one of page_id or discussion_id must be provided".to_string(),
                    ))
                } else {
                    let url = "https://api.notion.com/v1/comments";

                    let body = CreateCommentRequestBody {
                        parent: self.page_id.map(|page_id| {
                            crate::object::parent::Parent::PageParent(
                                crate::object::parent::PageParent {
                                    r#type: "page".to_string(),
                                    page_id,
                                },
                            )
                        }),
                        discussion_id: self.discussion_id,
                        rich_text,
                    };

                    let body_string = serde_json::to_string(&body)?;

                    println!("{}", body_string);

                    let request = self
                        .reqwest_client
                        .post(url)
                        .header("Content-Type", "application/json")
                        .body(body_string);

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

                    let comment = serde_json::from_slice::<crate::object::comment::Comment>(&body)?;

                    Ok(comment)
                }
            }
        }
    }
}
