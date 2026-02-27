/// Implement `Display` trait for a struct that has a field of type `String`.
///
///
/// ```ignore
/// #[async_trait::async_trait]
/// impl crate::r#trait::Paginate<notionrs_types::object::block::BlockResponse>
///     for GetBlockChildrenClient
/// {
///     fn paginate_start_cursor(self, start_cursor: Option<String>) -> Self {
///         match start_cursor {
///             Some(c) => self.start_cursor(c),
///             None => self,
///         }
///     }
///
///     async fn paginate_send(
///         self,
///     ) -> Result<
///         notionrs_types::object::response::ListResponse<
///             notionrs_types::object::block::BlockResponse,
///         >,
///         crate::error::Error,
///     > {
///         Ok(self.send().await?)
///     }
/// }
/// ```
/// ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓
/// ```ignore
/// crate::impl_paginate!(GetBlockChildrenClient, notionrs_types::object::block::BlockResponse);
/// ```
#[macro_export]
macro_rules! impl_paginate {
    ($struct_name:ty, $return_type:ty) => {
        impl crate::r#trait::Paginate<$return_type> for $struct_name {
            fn paginate_start_cursor(self, start_cursor: Option<String>) -> Self {
                match start_cursor {
                    Some(c) => self.start_cursor(c),
                    None => self,
                }
            }

            fn paginate_send(
                self,
            ) -> std::pin::Pin<
                Box<
                    dyn std::future::Future<
                            Output = Result<
                                notionrs_types::object::response::ListResponse<$return_type>,
                                crate::error::Error,
                            >,
                        > + Send
                        + Sync,
                >,
            > {
                Box::pin(async { Ok(self.send().await?) })
            }
        }
    };
}
