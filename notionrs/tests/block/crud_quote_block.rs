mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn crud_quote_block() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let block_id = std::env::var("NOTION_IT_CRUD_PAGE_ID").unwrap();

        let client = notionrs::Client::new();

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
            .block_id(block_id.clone())
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
