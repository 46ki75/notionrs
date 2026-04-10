mod integration_tests {

    use notionrs_types::prelude::*;

    /// [Heading 1](https://www.notion.so/33da03d79b268080bb41c56dbc0b790d)
    /// [Heading 2](https://www.notion.so/33ea03d79b2680c58f92d34a4d728ef0)
    /// [Heading 3](https://www.notion.so/33ea03d79b2680b29647eeffe1af73be)
    /// [Heading 4](https://www.notion.so/33ea03d79b2680bfa7cef85229dbe236)
    static PAGE_IDS: [&str; 4] = [
        "33da03d79b268080bb41c56dbc0b790d",
        "33ea03d79b2680c58f92d34a4d728ef0",
        "33ea03d79b2680b29647eeffe1af73be",
        "33ea03d79b2680bfa7cef85229dbe236",
    ];

    async fn crud_heading_block(level: u8) -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = RichText::from(format!("Heading{} !", level));

        let heading = HeadingBlock::default()
            .rich_text(vec![rich_text.clone()])
            .children(vec![])
            .is_toggleable(true);

        let block = match level {
            1 => Block::Heading1 { heading_1: heading },
            2 => Block::Heading2 { heading_2: heading },
            3 => Block::Heading3 { heading_3: heading },
            4 => Block::Heading4 { heading_4: heading },
            _ => panic!("Invalid heading level: {}", level),
        };

        let request = client
            .append_block_children()
            .block_id(PAGE_IDS[(level - 1) as usize])
            .children(vec![block]);

        let response = request.send().await?;

        // # --------------------------------------------------------------------------------
        //
        // get_block
        //
        // # --------------------------------------------------------------------------------

        let request = client
            .get_block()
            .block_id(response.results.first().unwrap().id.clone());

        let response = request.send().await?;

        // # --------------------------------------------------------------------------------
        //
        // update_block
        //
        // # --------------------------------------------------------------------------------

        let block = match response.block {
            Block::Heading1 { heading_1 } => {
                assert_eq!(heading_1.rich_text, vec![rich_text]);
                assert!(heading_1.is_toggleable);
                Block::Heading1 {
                    heading_1: heading_1.red().is_toggleable(false),
                }
            }
            Block::Heading2 { heading_2 } => {
                assert_eq!(heading_2.rich_text, vec![rich_text]);
                assert!(heading_2.is_toggleable);
                Block::Heading2 {
                    heading_2: heading_2.red().is_toggleable(false),
                }
            }
            Block::Heading3 { heading_3 } => {
                assert_eq!(heading_3.rich_text, vec![rich_text]);
                assert!(heading_3.is_toggleable);
                Block::Heading3 {
                    heading_3: heading_3.red().is_toggleable(false),
                }
            }
            Block::Heading4 { heading_4 } => {
                assert_eq!(heading_4.rich_text, vec![rich_text]);
                assert!(heading_4.is_toggleable);
                Block::Heading4 {
                    heading_4: heading_4.red().is_toggleable(false),
                }
            }
            e => panic!("{:?}", e),
        };

        let request = client
            .update_block()
            .block_id(response.id.clone())
            .block(block);

        let response = request.send().await?;

        println!("{:?}", response);

        // # --------------------------------------------------------------------------------
        //
        // delete_block
        //
        // # --------------------------------------------------------------------------------

        let request = client.delete_block().block_id(response.id.clone());

        let response = request.send().await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn crud_heading_1_block() -> Result<(), notionrs::Error> {
        crud_heading_block(1).await.unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn crud_heading_2_block() -> Result<(), notionrs::Error> {
        crud_heading_block(2).await.unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn crud_heading_3_block() -> Result<(), notionrs::Error> {
        crud_heading_block(3).await.unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn crud_heading_4_block() -> Result<(), notionrs::Error> {
        crud_heading_block(4).await.unwrap();
        Ok(())
    }
}
