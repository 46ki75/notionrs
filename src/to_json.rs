use serde::Serialize;
use serde_json;

pub trait ToJson {
    /// ## Usage:
    /// ```rs
    /// use notionrs::{client::NotionClient, error::NotionError, to_json::ToJson};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let result = notion.get_self().send().await?;
    ///     println!("{}", result.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// By importing `notionrs::prelude::*`, you can also use the `to_json()` method.
    ///
    /// ```rs
    ///use notionrs::{client::NotionClient, error::NotionError, prelude::*};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let result = notion.get_self().send().await?;
    ///     println!("{}", result.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```

    fn to_json(&self) -> String;
}

impl<T> ToJson for T
where
    T: Serialize,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
