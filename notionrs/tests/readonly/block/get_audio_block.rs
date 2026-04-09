mod integration_tests {

    use notionrs_types::prelude::*;

    // <https://www.notion.so/33da03d79b2680d6bbacd7734e565cd7>
    static PAGE_ID: &str = "33da03d79b2680d6bbacd7734e565cd7";

    #[tokio::test]
    async fn get_audio_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly"))
            .expect("Failed to load .env.readonly file");

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.get_block_children().block_id(PAGE_ID).send().await?;

        let audio_files = response
            .results
            .into_iter()
            .filter_map(|block_response| match block_response.block {
                Block::Audio { audio } => Some(audio),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(audio_files.len() > 0);

        println!("{:?}", audio_files);

        Ok(())
    }
}
