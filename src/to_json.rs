use serde::Serialize;
use serde_json;

pub trait ToJson {
    /// ## Usage:
    ///
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::Error;
    /// use notionrs::prelude::ToJson;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
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
    ///
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::Error;
    /// use notionrs::prelude::ToJson;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
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
