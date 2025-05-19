#[async_trait::async_trait]
pub trait Paginate<T>: Clone {
    fn paginate_start_cursor(self, start_cursor: Option<String>) -> Self;

    async fn paginate_send(
        self,
    ) -> Result<notionrs_types::object::response::ListResponse<T>, crate::error::Error>;
}
