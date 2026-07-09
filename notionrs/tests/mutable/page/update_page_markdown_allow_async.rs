mod integration_tests {

    use notionrs::client::page::update_page_markdown::UpdatePageMarkdownResponse;
    use notionrs_types::prelude::*;

    /// Maximum number of times to poll `get_async_task` before giving up.
    static MAX_POLL_ATTEMPTS: u32 = 30;

    /// Resolves an `update_page_markdown` response, polling `get_async_task` if
    /// Notion accepted the request asynchronously (`allow_async(true)`).
    /// Whether Notion actually goes async for a given request is
    /// server-decided, so this handles both outcomes.
    async fn resolve_markdown(
        client: &notionrs::Client,
        response: UpdatePageMarkdownResponse,
    ) -> Result<notionrs_types::object::page_markdown::PageMarkdownResponse, notionrs::Error> {
        let mut task = match response {
            UpdatePageMarkdownResponse::Markdown(markdown) => return Ok(markdown),
            UpdatePageMarkdownResponse::AsyncTask(task) => task,
        };

        for _ in 0..MAX_POLL_ATTEMPTS {
            task = match task {
                AsyncTaskResponse::Succeeded(succeeded) => {
                    // `succeeded.object`/`succeeded.id` describe the async task
                    // envelope itself (always "async_task" and the task's own
                    // ID) — the actual page_markdown fields are nested inside
                    // `succeeded.result`.
                    let id = succeeded
                        .result
                        .get("id")
                        .and_then(|value| value.as_str())
                        .unwrap_or_else(|| {
                            panic!(
                                "succeeded async task result missing `id`: {:?}",
                                succeeded.result
                            )
                        })
                        .to_string();

                    let markdown = succeeded
                        .result
                        .get("markdown")
                        .and_then(|value| value.as_str())
                        .unwrap_or_else(|| {
                            panic!(
                                "succeeded async task result missing `markdown`: {:?}",
                                succeeded.result
                            )
                        })
                        .to_string();

                    return Ok(
                        notionrs_types::object::page_markdown::PageMarkdownResponse {
                            object: "page_markdown".to_string(),
                            id,
                            markdown,
                            truncated: false,
                            unknown_block_ids: Vec::new(),
                            request_id: None,
                        },
                    );
                }
                AsyncTaskResponse::Failed(failed) => {
                    panic!("update_page_markdown async task failed: {:?}", failed.error);
                }
                AsyncTaskResponse::Queued(progress)
                | AsyncTaskResponse::Running(progress)
                | AsyncTaskResponse::Retrying(progress) => {
                    let wait_seconds = progress.poll_after_seconds.unwrap_or(1);
                    tokio::time::sleep(std::time::Duration::from_secs(wait_seconds)).await;
                    client.get_async_task().task_id(&progress.id).send().await?
                }
            };
        }

        panic!(
            "update_page_markdown async task did not complete within {MAX_POLL_ATTEMPTS} polling attempts"
        );
    }

    #[tokio::test]
    async fn update_page_markdown_allow_async() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // create a page
        let mut properties = std::collections::HashMap::new();
        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from(
                "Update Page Markdown Allow Async Test",
            )),
        );

        let page = client
            .create_page::<std::collections::HashMap<String, PageProperty>>()
            .properties(properties)
            .data_source_id(crate::mutable::DATA_SOURCE_ID)
            .send()
            .await?
            .into_page()?;

        // insert content using allow_async(true)
        let response = client
            .update_page_markdown()
            .page_id(&page.id)
            .insert_content("# Hello World\n\nInserted with `allow_async(true)`.")
            .allow_async(true)
            .send()
            .await?;

        let markdown = resolve_markdown(&client, response).await?;

        assert_eq!(markdown.object, "page_markdown");
        assert_eq!(markdown.id, page.id);

        // cleanup
        client.delete_block().block_id(&page.id).send().await?;

        Ok(())
    }
}
