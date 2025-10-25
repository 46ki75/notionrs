use notionrs_types::prelude::*;

pub mod block;
pub mod comment;
pub mod data_source;
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
    pub fn new(notion_api_key: impl AsRef<str>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        let secret = notion_api_key.as_ref().to_string();

        headers.insert(
            "Notion-Version",
            reqwest::header::HeaderValue::from_static("2025-09-03"),
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
    // Data Source
    //
    // # --------------------------------------------------------------------------------

    pub fn query_data_source(
        &self,
    ) -> crate::client::data_source::query_data_source::QueryDataSourceClient {
        crate::client::data_source::query_data_source::QueryDataSourceClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn create_data_source(
        &self,
    ) -> crate::client::data_source::create_data_source::CreateDataSourceClient {
        crate::client::data_source::create_data_source::CreateDataSourceClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn retrieve_data_source(
        &self,
    ) -> crate::client::data_source::retrieve_data_source::RetrieveDataSourceClient {
        crate::client::data_source::retrieve_data_source::RetrieveDataSourceClient {
            reqwest_client: self.reqwest_client.clone(),
            ..Default::default()
        }
    }

    pub fn update_data_source(
        &self,
    ) -> crate::client::data_source::update_data_source::UpdateDataSourceClient {
        crate::client::data_source::update_data_source::UpdateDataSourceClient {
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

    pub fn complete_file_upload(
        &self,
    ) -> crate::client::file_upload::complete_file_upload::CompleteFileUploadClient {
        crate::client::file_upload::complete_file_upload::CompleteFileUploadClient {
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

    /// **Experimental**
    pub async fn to_markdown<T>(&self, block_id: T) -> Result<Vec<String>, crate::error::Error>
    where
        T: AsRef<str>,
    {
        let mut markdown_list: Vec<String> = Vec::new();

        // Retrieve first-level children for the provided block id.
        let response = self
            .get_block_children()
            .block_id(block_id.as_ref())
            .send()
            .await?;

        // Stack holds (BlockResponse, indent_level); changed to stack for traversal
        let mut stack: Vec<(notionrs_types::prelude::BlockResponse, usize)> = Vec::new();

        for block in response.results.into_iter().rev() {
            stack.push((block, 0));
        }

        while let Some((block, indent)) = stack.pop() {
            match block.block {
                notionrs_types::prelude::Block::Audio { .. } => continue,
                notionrs_types::prelude::Block::Bookmark { bookmark } => {
                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}<{}>", prefix, bookmark.url));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Breadcrumb { .. } => continue,
                notionrs_types::prelude::Block::BulletedListItem { bulleted_list_item } => {
                    let text = bulleted_list_item
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}- {}", prefix, text));

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent + 1));
                        }
                    }
                }
                notionrs_types::prelude::Block::Callout { callout } => {
                    let text = callout
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}> {}", prefix, text));

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent));
                        }
                    }
                }
                notionrs_types::prelude::Block::ChildDatabase { .. } => continue,
                notionrs_types::prelude::Block::ChildPage { .. } => continue,
                notionrs_types::prelude::Block::Code { code } => {
                    let text = code
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    let language = code.language.to_string();
                    markdown_list.push(format!("{}```{}", prefix, language));
                    markdown_list.push(format!("{}{}", prefix, text));
                    markdown_list.push(format!("{}```", prefix));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::ColumnList { .. } => {
                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent));
                        }
                    }
                }
                notionrs_types::prelude::Block::Column { .. } => {
                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent));
                        }
                    }
                }
                notionrs_types::prelude::Block::Divider { .. } => {
                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}---", prefix));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Embed { embed } => {
                    let prefix = "  ".repeat(indent);
                    let url = embed.url;
                    markdown_list.push(format!("{}<{}>", prefix, url));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Equation { equation } => {
                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}$${}$$", prefix, equation.expression));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::File { file } => {
                    let prefix = "  ".repeat(indent);
                    let url = file.get_url();
                    markdown_list.push(format!("{}<{}>", prefix, url));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Heading1 { heading_1 } => {
                    let text = heading_1
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}# {}", prefix, text));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Heading2 { heading_2 } => {
                    let text = heading_2
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}## {}", prefix, text));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Heading3 { heading_3 } => {
                    let text = heading_3
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}### {}", prefix, text));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Image { image } => {
                    let prefix = "  ".repeat(indent);
                    let url = image.get_url();
                    let alt_text = match image {
                        File::External(external_file) => external_file
                            .caption
                            .unwrap_or_default()
                            .into_iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(""),
                        File::NotionHosted(notion_hosted_file) => notion_hosted_file
                            .caption
                            .unwrap_or_default()
                            .into_iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(""),
                        File::ApiUploaded(..) => String::new(),
                        _ => String::new(),
                    };
                    markdown_list.push(format!("{}![{}]({})", prefix, alt_text, url));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::LinkPreview { link_preview } => {
                    let prefix = "  ".repeat(indent);
                    let url = link_preview.url;
                    markdown_list.push(format!("{}<{}>", prefix, url));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::NumberedListItem { numbered_list_item } => {
                    let text = numbered_list_item
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}1. {}", prefix, text));

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent + 1));
                        }
                    }
                }
                notionrs_types::prelude::Block::Paragraph { paragraph } => {
                    let text = paragraph
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}{}", prefix, text));
                    markdown_list.push(String::new()); // New Line

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent));
                        }
                    }
                }
                notionrs_types::prelude::Block::Pdf { pdf } => {
                    let prefix = "  ".repeat(indent);
                    let url = pdf.get_url();
                    markdown_list.push(format!("{}<{}>", prefix, url));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Quote { quote } => {
                    let text = quote
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}> {}", prefix, text));

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent));
                        }
                    }

                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::SyncedBlock { synced_block } => {
                    let block_id_to_fetch = synced_block
                        .clone()
                        .synced_from
                        .map(|s| s.block_id)
                        .unwrap_or_else(|| block.id.clone());

                    if block.has_children || synced_block.synced_from.is_some() {
                        if let Ok(children_resp) = self
                            .get_block_children()
                            .block_id(&block_id_to_fetch)
                            .send()
                            .await
                        {
                            for child in children_resp.results.into_iter().rev() {
                                stack.push((child, indent));
                            }
                        }
                    }
                }
                notionrs_types::prelude::Block::TableOfContents { .. } => continue,
                notionrs_types::prelude::Block::Table { .. } => {
                    let prefix = "  ".repeat(indent);

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        let rows: Vec<_> = children_resp.results;

                        if rows.is_empty() {
                            continue;
                        }

                        let mut table_rows = Vec::new();
                        for row_block in rows {
                            if let notionrs_types::prelude::Block::TableRow { table_row } =
                                row_block.block
                            {
                                let cells: Vec<String> = table_row
                                    .cells
                                    .into_iter()
                                    .map(|cell| {
                                        cell.into_iter()
                                            .map(|t| t.to_markdown())
                                            .collect::<String>()
                                    })
                                    .collect();
                                table_rows.push(cells);
                            }
                        }

                        if table_rows.is_empty() {
                            continue;
                        }

                        if let Some(header) = table_rows.first() {
                            markdown_list.push(format!("{}| {} |", prefix, header.join(" | ")));

                            let separator = vec!["---"; header.len()].join(" | ");
                            markdown_list.push(format!("{}| {} |", prefix, separator));
                        }

                        for row in table_rows.iter().skip(1) {
                            markdown_list.push(format!("{}| {} |", prefix, row.join(" | ")));
                        }

                        markdown_list.push(String::new());
                    }
                }
                notionrs_types::prelude::Block::TableRow { .. } => {
                    continue;
                }
                notionrs_types::prelude::Block::Template { .. } => continue,
                notionrs_types::prelude::Block::ToDo { to_do } => {
                    let text = to_do
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    let checkbox = if to_do.checked { "[x]" } else { "[ ]" };
                    markdown_list.push(format!("{}- {} {}", prefix, checkbox, text));

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent + 1));
                        }
                    }
                }
                notionrs_types::prelude::Block::Toggle { toggle } => {
                    let text = toggle
                        .rich_text
                        .into_iter()
                        .map(|t| t.to_markdown())
                        .collect::<String>();

                    let prefix = "  ".repeat(indent);
                    markdown_list.push(format!("{}> **{}**", prefix, text));

                    if block.has_children {
                        let children_resp =
                            self.get_block_children().block_id(&block.id).send().await?;
                        for child in children_resp.results.into_iter().rev() {
                            stack.push((child, indent));
                        }
                    }

                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Video { video } => {
                    let prefix = "  ".repeat(indent);
                    let url = video.get_url();
                    markdown_list.push(format!("{}<{}>", prefix, url));
                    markdown_list.push(String::new()); // New Line
                }
                notionrs_types::prelude::Block::Unsupported => continue,
            };
        }

        Ok(markdown_list)
    }
}
