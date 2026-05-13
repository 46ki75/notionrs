mod integration_tests {

    use notionrs_types::object::request::meeting_notes::{
        MeetingNotesCombinatorFilter, MeetingNotesCombinatorOperator, MeetingNotesFilterNode,
        MeetingNotesPropertyFilter, MeetingNotesPropertyFilterCondition, MeetingNotesPropertyName,
        MeetingNotesSort, MeetingNotesSortDirection,
    };

    // # --------------------------------------------------------------------------------
    //
    // query_meeting_notes
    //
    // # --------------------------------------------------------------------------------

    /// Basic query with no filter or sort.
    #[tokio::test]
    #[ignore]
    async fn query_meeting_notes() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.query_meeting_notes().send().await?;

        println!("{:?}", response);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // query_meeting_notes with limit
    //
    // # --------------------------------------------------------------------------------

    /// Query with a limit on the number of results.
    #[tokio::test]
    #[ignore]
    async fn query_meeting_notes_with_limit() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.query_meeting_notes().limit(10).send().await?;

        println!("{:?}", response);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // query_meeting_notes with filter
    //
    // # --------------------------------------------------------------------------------

    /// Query filtered by title using a string_contains operator.
    #[tokio::test]
    #[ignore]
    async fn query_meeting_notes_with_filter() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let filter = MeetingNotesCombinatorFilter {
            operator: MeetingNotesCombinatorOperator::And,
            filters: Some(vec![MeetingNotesFilterNode::Property(
                MeetingNotesPropertyFilter {
                    property: MeetingNotesPropertyName::Title,
                    filter: MeetingNotesPropertyFilterCondition {
                        operator: "is_not_empty".to_string(),
                        value: None,
                    },
                },
            )]),
        };

        let response = client.query_meeting_notes().filter(filter).send().await?;

        println!("{:?}", response);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // query_meeting_notes with sort
    //
    // # --------------------------------------------------------------------------------

    /// Query sorted by created_time descending.
    #[tokio::test]
    #[ignore]
    async fn query_meeting_notes_with_sort() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let sort = vec![MeetingNotesSort {
            property: MeetingNotesPropertyName::CreatedTime,
            direction: MeetingNotesSortDirection::Descending,
        }];

        let response = client.query_meeting_notes().sort(sort).send().await?;

        println!("{:?}", response);

        Ok(())
    }
}
