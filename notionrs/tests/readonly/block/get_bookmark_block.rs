mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/33da03d79b268081890bd6ec66905981>
    static PAGE_ID: &str = "33da03d79b268081890bd6ec66905981";

    #[tokio::test]
    async fn get_audio_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let bookmark_blocks = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Bookmark { bookmark } => Some(bookmark),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(bookmark_blocks.len() > 0);

        println!("{:?}", bookmark_blocks);

        Ok(())
    }
}
