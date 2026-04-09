mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b26803694f1fd9a8d867184>
    static PAGE_ID: &str = "33da03d79b26803694f1fd9a8d867184";

    #[tokio::test]
    async fn upload_file() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let id = client
            .create_file_upload()
            .mode(notionrs::client::file_upload::create_file_upload::FileUplpadMode::SinglePart)
            .send()
            .await?
            .id;

        let file = include_bytes!("../../../Cargo.toml");

        let file_upload_id = client
            .send_file_upload()
            .file_upload_id(id)
            .file(file.to_vec())
            .send()
            .await?
            .id;

        let file: FileUpload = client
            .retrieve_file_upload()
            .file_upload_id(file_upload_id)
            .send()
            .await?;

        // Create a file block with the uploaded file

        let file_block = Block::File {
            file: File::ApiUploaded(ApiUploadedFile {
                file_upload: ApiUploadedFileParameter { id: file.id },
            }),
        };

        let response = client
            .append_block_children()
            .block_id(PAGE_ID)
            .children(vec![file_block])
            .send()
            .await?;

        println!("{:#?}", response);

        Ok(())
    }
}
