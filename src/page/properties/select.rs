use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#select>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"select"`
/// - `$.['*'].select`: Select object (optional)
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example select page property value
///
/// ```json
/// {
///   "Select": {
///     "type": "select",
///     "id": "chOy",
///     "select": {
///       "id": "eede87ce-52db-4b16-9931-2bc40687d697",
///       "name": "TODO",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PageSelectProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Select object (optional)
    pub select: Option<crate::others::select::Select>,
}

impl PageSelectProperty {
    pub fn select(mut self, select: crate::others::select::Select) -> Self {
        self.select = Some(select);
        self
    }
}

impl<T> From<T> for PageSelectProperty
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let select = crate::others::select::Select::from(value);
        Self::default().select(select)
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
    fn deserialize_select_property() {
        let json_data = r#"
        {
            "Select": {
                "type": "select",
                "id": "chOy",
                "select": {
                    "id": "eede87ce-52db-4b16-9931-2bc40687d697",
                    "name": "TODO",
                    "color": "default"
                }
            }
        }
        "#;

        let select_map = serde_json::from_str::<
            std::collections::HashMap<String, PageSelectProperty>,
        >(json_data)
        .unwrap();

        let select = select_map.get("Select").unwrap();

        assert_eq!(select.id, Some("chOy".to_string()));
        assert_eq!(
            select.select.as_ref().unwrap().id,
            Some("eede87ce-52db-4b16-9931-2bc40687d697".to_string())
        );
        assert_eq!(select.select.as_ref().unwrap().name, "TODO");
        assert_eq!(
            select.select.as_ref().unwrap().color,
            Some(crate::others::select::SelectColor::Default)
        );
    }
}
