mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn upload_file() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let secret = std::env::var("NOTION_TOKEN").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new().secret(secret);

        let id = client
            .create_file_upload()
            .mode(notionrs::client::file_upload::create_file_upload::FileUplpadMode::SinglePart)
            .send()
            .await?
            .id;

        let file = include_bytes!("../../Cargo.toml");

        let file_upload_id = client
            .send_file_upload()
            .file_upload_id(id)
            .file(file.to_vec())
            .send()
            .await?
            .id;

        let _: FileUpload = client
            .retrieve_file_upload()
            .file_upload_id(file_upload_id)
            .send()
            .await?;

        Ok(())
    }
}
