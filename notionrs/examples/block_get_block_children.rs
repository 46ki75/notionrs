use notionrs::{
    Client,
    error::Error,
    object::block::{Block, BlockResponse},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request: notionrs::client::block::get_block_children::GetBlockChildrenClient = client
        .get_block_children()
        .block_id("BLOCK_ID")
        .page_size(100);

    let response = request.send().await?;

    for BlockResponse {
        object: _,
        id: _,
        parent: _,
        created_time: _,
        last_edited_time: _,
        created_by: _,
        last_edited_by: _,
        has_children: _,
        archived: _,
        in_trash: _,
        block,
    } in response.results
    {
        match block {
            Block::Audio { audio: _ } => todo!(),
            Block::Bookmark { bookmark: _ } => todo!(),
            Block::Breadcrumb { breadcrumb: _ } => todo!(),
            Block::BulletedListItem {
                bulleted_list_item: _,
            } => todo!(),
            Block::Callout { callout: _ } => todo!(),
            Block::ChildDatabase { child_database: _ } => todo!(),
            Block::ChildPage { child_page: _ } => todo!(),
            Block::Code { code: _ } => todo!(),
            Block::ColumnList { column_list: _ } => todo!(),
            Block::Column { column: _ } => todo!(),
            Block::Divider { divider: _ } => todo!(),
            Block::Embed { embed: _ } => todo!(),
            Block::Equation { equation: _ } => todo!(),
            Block::File { file: _ } => todo!(),
            Block::Heading1 { heading_1: _ } => todo!(),
            Block::Heading2 { heading_2: _ } => todo!(),
            Block::Heading3 { heading_3: _ } => todo!(),
            Block::Image { image: _ } => todo!(),
            Block::LinkPreview { link_preview: _ } => todo!(),
            Block::NumberedListItem {
                numbered_list_item: _,
            } => todo!(),
            Block::Paragraph { paragraph: _ } => todo!(),
            Block::Pdf { pdf: _ } => todo!(),
            Block::Quote { quote: _ } => todo!(),
            Block::SyncedBlock { synced_block: _ } => todo!(),
            Block::Table { table: _ } => todo!(),
            Block::TableRow { table_row: _ } => todo!(),
            Block::Template { template: _ } => todo!(),
            Block::ToDo { to_do: _ } => todo!(),
            Block::Toggle { toggle: _ } => todo!(),
            Block::Video { video: _ } => todo!(),
            Block::Unknown(_) => todo!(),
        }
    }

    Ok(())
}
