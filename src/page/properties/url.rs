use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#url>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"url"`
/// - `$.['*'].url`: A string that describes a web address.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example url page property value
///
/// ```json
/// {
///   "URL": {
///     "type": "url",
///     "id": "h_AH",
///     "url": "https://developers.notion.com/reference/page-property-values#url"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageUrlProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// A string that describes a web address.
    pub url: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn deserialize_url_property() {
        let json_data = r#"
        {
            "URL": {
                "type": "url",
                "id": "h_AH",
                "url": "https://developers.notion.com/reference/page-property-values#url"
            }
        }
        "#;

        let url_map =
            serde_json::from_str::<std::collections::HashMap<String, PageUrlProperty>>(json_data)
                .unwrap();

        let url = url_map.get("URL").unwrap();

        assert_eq!(url.id, "h_AH");
        assert_eq!(
            url.url,
            Some("https://developers.notion.com/reference/page-property-values#url".to_string())
        );
    }
}
