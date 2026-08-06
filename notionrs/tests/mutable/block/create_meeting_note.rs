mod integration_tests {
    use notionrs::client::file_upload::create_file_upload::FileUploadMode;
    use notionrs_types::prelude::CreateMeetingNoteLanguage;

    /// <https://www.notion.so/33da03d79b26803694f1fd9a8d867184>
    static PAGE_ID: &str = "33da03d79b26803694f1fd9a8d867184";

    fn silent_wav() -> Vec<u8> {
        const SAMPLE_RATE: u32 = 8_000;
        const SECONDS: u32 = 10;
        const BITS_PER_SAMPLE: u16 = 16;
        const CHANNELS: u16 = 1;

        let data_size = SAMPLE_RATE * SECONDS * u32::from(BITS_PER_SAMPLE / 8);
        let mut wav = Vec::with_capacity(44 + data_size as usize);
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(36 + data_size).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&CHANNELS.to_le_bytes());
        wav.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
        wav.extend_from_slice(&(SAMPLE_RATE * u32::from(BITS_PER_SAMPLE / 8)).to_le_bytes());
        wav.extend_from_slice(&(CHANNELS * (BITS_PER_SAMPLE / 8)).to_le_bytes());
        wav.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_size.to_le_bytes());
        wav.resize(44 + data_size as usize, 0);
        wav
    }

    #[tokio::test]
    #[ignore = "requires a Notion plan with AI meeting notes enabled"]
    async fn create_meeting_note_from_file_upload() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let file_upload = client
            .create_file_upload()
            .mode(FileUploadMode::SinglePart)
            .filename("meeting-note-test.wav")
            .content_type("audio/wav")
            .send()
            .await?;
        let file_upload = client
            .send_file_upload()
            .file_upload_id(file_upload.id)
            .file(silent_wav())
            .filename("meeting-note-test.wav")
            .content_type("audio/wav")
            .send()
            .await?;

        let response = client
            .create_meeting_note()
            .file_upload_source(file_upload.id)
            .parent_page_id(PAGE_ID)
            .title("notionrs integration test")
            .language(CreateMeetingNoteLanguage::English)
            .kickoff_summary(false)
            .send()
            .await?;
        let block_id = response.id().to_owned();

        client.delete_block().block_id(&block_id).send().await?;

        assert!(!block_id.is_empty());
        Ok(())
    }
}
