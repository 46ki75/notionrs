# NotionRs Webhooks

This crate provides event deserialization and signature verification features for Notion webhooks.

Minimal example:

```rs
use axum::{response::IntoResponse, routing::post};
use http::{StatusCode, request::Parts};
use notionrs_webhooks::PageContentUpdated;

async fn handler(parts: Parts, body: axum::body::Bytes) -> impl IntoResponse {
    let verification_token: &'static [u8; 5] = b"token";
    let maybe_x_notion_signature = parts.headers.get("x-api-signature");

    match maybe_x_notion_signature {
        None => return StatusCode::UNAUTHORIZED.into_response(),
        Some(x_notion_signature) => {
            let is_valid = notionrs_webhooks::verify_signature(
                verification_token,
                &body,
                x_notion_signature.as_bytes(),
            );

            if !is_valid {
                return StatusCode::FORBIDDEN.into_response();
            };

            let event = serde_json::from_slice::<notionrs_webhooks::WebhookEvent>(&body).unwrap();

            return match event.data {
                notionrs_webhooks::EventData::PageContentUpdated(PageContentUpdated {
                    parent: _,
                    updated_blocks: _,
                }) => {
                    // Something to do.
                    StatusCode::OK.into_response()
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };
        }
    };
}

#[tokio::main]
async fn main() {
    let router = axum::Router::new().route("/", post(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
```
