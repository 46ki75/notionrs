/// <https://developers.notion.com/reference/retrieve-an-async-task>
#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct GetAsyncTaskClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The ID of the async task to retrieve.
    pub(crate) task_id: Option<String>,
}

impl GetAsyncTaskClient {
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::async_task::AsyncTaskResponse, crate::error::Error> {
        let task_id = self.task_id.ok_or(crate::error::Error::RequestParameter(
            "`task_id` is not set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/async_tasks/{}", task_id);

        let request = self.reqwest_client.get(url);

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

        let task =
            serde_json::from_slice::<notionrs_types::object::async_task::AsyncTaskResponse>(&body)?;

        Ok(task)
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
    fn get_async_task_client_task_id_setter() {
        let client = GetAsyncTaskClient::default().task_id("task-id-123");
        assert_eq!(client.task_id, Some("task-id-123".to_string()));
    }

    #[tokio::test]
    async fn get_async_task_client_rejects_missing_task_id() {
        let client = GetAsyncTaskClient::default();
        let result = client.send().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::Error::RequestParameter(msg) => {
                assert!(msg.contains("task_id"));
            }
            other => panic!("Expected RequestParameter error, got: {:?}", other),
        }
    }
}
