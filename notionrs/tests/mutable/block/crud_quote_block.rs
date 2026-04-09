mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b26809cb6dcdda7293da155>
    static PAGE_ID: &str = "33da03d79b26809cb6dcdda7293da155";

    #[tokio::test]
    async fn crud_quote_block() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.mutable")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // append_block_children
        //
        // # --------------------------------------------------------------------------------

        let rich_text = RichText::from("rich text");

        let children = vec![Block::Paragraph {
            paragraph: ParagraphBlock::default()
                .rich_text(vec![rich_text.clone()])
                .blue_background(),
        }];

        let block = Block::Quote {
            quote: QuoteBlock::default()
                .rich_text(vec![rich_text.clone()])
                .blue_background()
                .children(children),
        };

        let request = client
            .append_block_children()
            .block_id(PAGE_ID)
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
            Block::Quote { quote } => {
                assert_eq!(quote.rich_text, vec![rich_text]);
                assert_eq!(
                    quote.color,
                    notionrs_types::object::color::Color::BlueBackground
                );
                Block::Quote {
                    quote: quote.green_background(),
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
}
