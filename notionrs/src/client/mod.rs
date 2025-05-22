pub mod block;
pub mod comment;
pub mod database;
pub mod file_upload;
pub mod page;
pub mod search;
pub mod user;

#[derive(Default, Debug)]
pub struct Client {
    reqwest_client: reqwest::Client,
}

impl Client {
    // TODO: docs: new method
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        let secret = std::env::var("NOTION_TOKEN").unwrap_or_else(|_| String::new());

        headers.insert(
            "Notion-Version",
            reqwest::header::HeaderValue::from_static("2022-06-28"),
        );
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", secret))
                .expect("Invalid header value"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Client {
            reqwest_client: client,
        }
    }

    /// This method sets the token used for calling the Notion API.
    /// If you don't set it, the client will automatically read
    /// and use the environment variable named `NOTION_TOKEN` during initialization.
    ///
    /// For details on obtaining a Notion token, please refer to the
    /// [Notion Developer Documentation](https://developers.notion.com/docs/authorization).
    ///
    /// ```no_run
    /// use notionrs::client::Client;
    /// // ...
    /// let client = Client::new().secret("secret_XXXXXXXXXXXXXX");
    /// ```
    pub fn secret<T>(mut self, notion_api_key: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut headers = reqwest::header::HeaderMap::new();
        let secret = notion_api_key.as_ref().to_string();

        headers.insert(
            "Notion-Version",
            reqwest::header::HeaderValue::from_static("2022-06-28"),
        );
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", secret))
                .expect("Invalid header value"),
        );

        self.reqwest_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        self
    }

    // # --------------------------------------------------------------------------------
    //
    // User
    //
    // # --------------------------------------------------------------------------------

    // TODO: docs: list_users method
    pub fn list_users(&self) -> crate::client::user::list_users::ListUsersClient {
        crate::client::user::list_users::ListUsersClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs: get_user method
    pub fn get_user(&self) -> crate::client::user::get_user::GetUserClient {
        crate::client::user::get_user::GetUserClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs: get_self method
    pub fn get_self(&self) -> crate::client::user::get_self::GetSelfClient {
        crate::client::user::get_self::GetSelfClient {
            reqwest_client: self.reqwest_client.clone(),
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Page
    //
    // # --------------------------------------------------------------------------------

    // TODO: docs: get_page method
    pub fn get_page(&self) -> crate::client::page::get_page::GetPageClient {
        crate::client::page::get_page::GetPageClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs: get_page_property_item method
    pub fn get_page_property_item(
        &self,
    ) -> crate::client::page::get_page_property_item::GetPagePropertyItemClient {
        crate::client::page::get_page_property_item::GetPagePropertyItemClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn create_page(&self) -> crate::client::page::create_page::CreatePageClient {
        crate::client::page::create_page::CreatePageClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn update_page(&self) -> crate::client::page::update_page::UpdatePageClient {
        crate::client::page::update_page::UpdatePageClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Database
    //
    // # --------------------------------------------------------------------------------

    // TODO: docs
    pub fn query_database(&self) -> crate::client::database::query_database::QueryDatabaseClient {
        crate::client::database::query_database::QueryDatabaseClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn create_database(
        &self,
    ) -> crate::client::database::create_database::CreateDatabaseClient {
        crate::client::database::create_database::CreateDatabaseClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn update_database(
        &self,
    ) -> crate::client::database::update_database::UpdateDatabaseClient {
        crate::client::database::update_database::UpdateDatabaseClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn retrieve_database(
        &self,
    ) -> crate::client::database::retrieve_database::RetrieveDatabaseClient {
        crate::client::database::retrieve_database::RetrieveDatabaseClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Block
    //
    // # --------------------------------------------------------------------------------

    // TODO: docs
    pub fn get_block(&self) -> crate::client::block::get_block::GetBlockClient {
        crate::client::block::get_block::GetBlockClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs
    pub fn get_block_children(
        &self,
    ) -> crate::client::block::get_block_children::GetBlockChildrenClient {
        crate::client::block::get_block_children::GetBlockChildrenClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs
    pub fn delete_block(&self) -> crate::client::block::delete_block::DeleteBlockClient {
        crate::client::block::delete_block::DeleteBlockClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs
    pub fn append_block_children(
        &self,
    ) -> crate::client::block::append_block_children::AppendBlockChildrenClient {
        crate::client::block::append_block_children::AppendBlockChildrenClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs
    pub fn update_block(&self) -> crate::client::block::update_block::UpdateBlockClient {
        crate::client::block::update_block::UpdateBlockClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // search
    //
    // # --------------------------------------------------------------------------------

    // TODO: docs
    pub fn search(&self) -> crate::client::search::SearchClient {
        crate::client::search::SearchClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs
    pub fn search_database(&self) -> crate::client::search::SearchDatabaseClient {
        crate::client::search::SearchDatabaseClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs
    pub fn search_page(&self) -> crate::client::search::SearchPageClient {
        crate::client::search::SearchPageClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // comment
    //
    // # --------------------------------------------------------------------------------

    // TODO: docs
    pub fn create_comment(&self) -> crate::client::comment::create_comment::CreateCommentClient {
        crate::client::comment::create_comment::CreateCommentClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // TODO: docs
    pub fn retrieve_comments(
        &self,
    ) -> crate::client::comment::retrieve_comments::RetrieveCommentsClient {
        crate::client::comment::retrieve_comments::RetrieveCommentsClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // File Upload
    //
    // # --------------------------------------------------------------------------------

    pub fn create_file_upload(
        &self,
    ) -> crate::client::file_upload::create_file_upload::CreateFileUploadClient {
        crate::client::file_upload::create_file_upload::CreateFileUploadClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn send_file_upload(
        &self,
    ) -> crate::client::file_upload::send_file_upload::SendFileUploadClient {
        crate::client::file_upload::send_file_upload::SendFileUploadClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn retrieve_file_upload(
        &self,
    ) -> crate::client::file_upload::retrieve_file_upload::RetrieveFileUploadClient {
        crate::client::file_upload::retrieve_file_upload::RetrieveFileUploadClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn list_file_uploads(
        &self,
    ) -> crate::client::file_upload::list_file_uploads::ListFileUploadClient {
        crate::client::file_upload::list_file_uploads::ListFileUploadClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub async fn paginate<C, T>(client: C) -> Result<Vec<T>, crate::error::Error>
    where
        C: crate::r#trait::Paginate<T> + Clone + Send + 'static,
        T: Send + 'static,
    {
        use futures::TryStreamExt;
        let results = Self::paginate_stream(client).try_collect().await?;
        Ok(results)
    }

    pub fn paginate_stream<C, T>(
        client: C,
    ) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<T, crate::error::Error>> + Send>>
    where
        C: crate::r#trait::Paginate<T> + Clone + Send + 'static,
        T: Send + 'static,
    {
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
