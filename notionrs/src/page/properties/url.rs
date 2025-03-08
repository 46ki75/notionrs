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
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PageUrlProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing)]
    pub id: Option<String>,

    /// A string that describes a web address.
    pub url: Option<String>,
}

impl PageUrlProperty {
    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.url = Some(url.as_ref().to_string());
        self
    }
}

impl<T> From<T> for PageUrlProperty
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self::default().url(value)
    }
}

impl std::fmt::Display for PageUrlProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.url.as_ref().unwrap())
    }
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

        assert_eq!(url.id, Some("h_AH".to_string()));
        assert_eq!(
            url.url,
            Some("https://developers.notion.com/reference/page-property-values#url".to_string())
        );
    }
}
