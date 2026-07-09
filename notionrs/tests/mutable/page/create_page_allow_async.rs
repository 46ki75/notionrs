mod integration_tests {

    use notionrs::client::page::create_page::CreatePageResponse;
    use notionrs_types::prelude::*;

    /// Maximum number of times to poll `get_async_task` before giving up.
    static MAX_POLL_ATTEMPTS: u32 = 30;

    /// Resolves a `create_page` response to the created page's ID, polling
    /// `get_async_task` if Notion accepted the request asynchronously
    /// (`allow_async(true)`). Whether Notion actually goes async for a given
    /// request is server-decided, so this handles both outcomes.
    async fn resolve_created_page_id(
        client: &notionrs::Client,
        response: CreatePageResponse<std::collections::HashMap<String, PageProperty>>,
    ) -> Result<String, notionrs::Error> {
        let mut task = match response {
            CreatePageResponse::Page(page) => return Ok(page.id),
            CreatePageResponse::AsyncTask(task) => task,
        };

        for _ in 0..MAX_POLL_ATTEMPTS {
            task = match task {
                AsyncTaskResponse::Succeeded(succeeded) => {
                    return Ok(succeeded
                        .result
                        .get("id")
                        .and_then(|value| value.as_str())
                        .unwrap_or_else(|| {
                            panic!(
                                "succeeded async task result missing `id`: {:?}",
                                succeeded.result
                            )
                        })
                        .to_string());
                }
                AsyncTaskResponse::Failed(failed) => {
                    panic!("create_page async task failed: {:?}", failed.error);
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
            "create_page async task did not complete within {MAX_POLL_ATTEMPTS} polling attempts"
        );
    }

    #[tokio::test]
    async fn create_page_allow_async() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("Create Page Allow Async Test")),
        );

        let response = client
            .create_page::<std::collections::HashMap<String, PageProperty>>()
            .properties(properties)
            .data_source_id(crate::mutable::DATA_SOURCE_ID)
            .markdown("# Hello World\n\nCreated with `allow_async(true)`.")
            .allow_async(true)
            .send()
            .await?;

        let page_id = resolve_created_page_id(&client, response).await?;

        // cleanup
        client.delete_block().block_id(page_id).send().await?;

        Ok(())
    }
}
