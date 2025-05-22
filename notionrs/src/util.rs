pub async fn send_and_convert<ReturnType>(
    request_builder: reqwest::RequestBuilder,
) -> Result<ReturnType, crate::error::Error>
where
    ReturnType: for<'a> serde::Deserialize<'a>,
{
    let response = request_builder
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

    let parsed = serde_json::from_slice::<ReturnType>(&body)?;

    Ok(parsed)
}
