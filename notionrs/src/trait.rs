use std::{future::Future, pin::Pin};

pub trait Paginate<T>: Clone + Send + 'static {
    fn paginate_start_cursor(self, start_cursor: Option<String>) -> Self;

    fn paginate_send(
        self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        notionrs_types::object::response::ListResponse<T>,
                        crate::error::Error,
                    >,
                > + Send
                + Sync,
        >,
    >;
}

pub trait PaginateExt<T>: Paginate<T> {
    /// Returns an async stream that yields all items across paginated responses.
    ///
    /// This method automatically fetches and streams all pages, handling pagination internally.
    /// Each item is yielded as soon as it is available.
    ///
    /// ```ignore
    /// use futures::TryStreamExt;
    /// use notionrs::r#trait::PaginateExt;
    /// use notionrs_types::prelude::*;
    ///
    /// let response: Vec<PageResponse> = client
    ///     .query_data_source()
    ///     .data_source_id(data_source_id)
    ///     .into_stream()
    ///     .try_collect()
    ///     .await
    ///     .unwrap();
    /// ```
    fn into_stream(
        self,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<T, crate::error::Error>> + Send + 'static>,
    >
    where
        T: Send + 'static;
}

impl<T, U> PaginateExt<T> for U
where
    U: Paginate<T> + Send + 'static,
{
    fn into_stream(
        self,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<T, crate::error::Error>> + Send + 'static>,
    >
    where
        T: Send + 'static,
    {
        let client = self.clone();

        Box::pin(futures::stream::try_unfold(
            (client, None::<String>, true, Vec::<T>::new().into_iter()),
            |(client, next_cursor, has_more, mut buffer)| async move {
                if let Some(item) = buffer.next() {
                    return Ok(Some((item, (client, next_cursor, has_more, buffer))));
                } else if !has_more {
                    return Ok(None);
                };

                let response = client
                    .clone()
                    .paginate_start_cursor(next_cursor)
                    .paginate_send()
                    .await?;

                let mut new_buffer = response.results.into_iter();

                let maybe_first_item = new_buffer.next();

                let state = (
                    client,
                    response.next_cursor,
                    response.has_more.unwrap_or_default(),
                    new_buffer,
                );

                match maybe_first_item {
                    Some(first_item) => Ok(Some((first_item, state))),
                    None => Ok(None),
                }
            },
        ))
    }
}
