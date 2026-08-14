pub mod create_page;
pub mod get_page;
pub mod get_page_markdown;
pub mod get_page_property_item;
pub mod move_page;
pub mod update_page;
pub mod update_page_markdown;

pub(super) fn with_filter_properties(
    request: reqwest::RequestBuilder,
    filter_properties: Option<Vec<String>>,
) -> reqwest::RequestBuilder {
    let Some(filter_properties) = filter_properties else {
        return request;
    };

    let query = filter_properties
        .into_iter()
        .map(|property| {
            let property = percent_encoding::percent_decode_str(&property)
                .decode_utf8_lossy()
                .into_owned();
            ("filter_properties", property)
        })
        .collect::<Vec<_>>();

    request.query(&query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_properties_are_encoded_as_repeated_query_parameters() {
        let request = reqwest::Client::new().post("https://api.notion.com/v1/pages");
        let request = with_filter_properties(
            request,
            Some(vec!["title".to_string(), "%3AUj%3B".to_string()]),
        )
        .build()
        .unwrap();

        assert_eq!(
            request.url().query(),
            Some("filter_properties=title&filter_properties=%3AUj%3B")
        );
    }

    #[test]
    fn absent_filter_properties_do_not_add_a_query_string() {
        let request = reqwest::Client::new().post("https://api.notion.com/v1/pages");
        let request = with_filter_properties(request, None).build().unwrap();

        assert_eq!(request.url().query(), None);
    }
}
