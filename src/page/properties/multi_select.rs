use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#multi-select>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"multi_select"`
/// - `$.['*'].multi_select`: Array of Select object
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example multi_select page property value
///
/// ```json
/// {
///   "Multi-select": {
///     "id": "_bnY",
///     "type": "multi_select",
///     "multi_select": [
///       {
///         "id": "959ba3e3-5a64-4ee6-864b-9e94ddc024d5",
///         "name": "HTML",
///         "color": "orange"
///       },
///       {
///         "id": "f22b05c9-0225-4dee-b25b-db7e63a47e0b",
///         "name": "CSS",
///         "color": "blue"
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PageMultiSelectProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Array of Select object
    pub multi_select: Vec<crate::others::select::Select>,
}

impl PageMultiSelectProperty {
    pub fn multi_select(mut self, multi_select: Vec<crate::others::select::Select>) -> Self {
        self.multi_select = multi_select;
        self
    }
}

impl std::fmt::Display for PageMultiSelectProperty {
    /// Display
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.multi_select
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        )
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
    fn deserialize_page_multi_select_property() {
        let json_data = r#"
        {
            "Multi-select": {
                "id": "_bnY",
                "type": "multi_select",
                "multi_select": [
                    {
                        "id": "959ba3e3-5a64-4ee6-864b-9e94ddc024d5",
                        "name": "HTML",
                        "color": "orange"
                    },
                    {
                        "id": "f22b05c9-0225-4dee-b25b-db7e63a47e0b",
                        "name": "CSS",
                        "color": "blue"
                    }
                ]
            }
        }
        "#;

        let multi_select_map = serde_json::from_str::<
            std::collections::HashMap<String, PageMultiSelectProperty>,
        >(json_data)
        .unwrap();

        let multi_select = multi_select_map.get("Multi-select").unwrap();

        assert_eq!(multi_select.id, Some("_bnY".to_string()));

        assert_eq!(
            multi_select.multi_select.first().unwrap().id,
            Some("959ba3e3-5a64-4ee6-864b-9e94ddc024d5".to_string())
        );
        assert_eq!(multi_select.multi_select.first().unwrap().name, "HTML");
        assert_eq!(
            multi_select.multi_select.first().unwrap().color,
            Some(crate::others::select::SelectColor::Orange)
        );

        assert_eq!(
            multi_select.multi_select.get(1).unwrap().id,
            Some("f22b05c9-0225-4dee-b25b-db7e63a47e0b".to_string())
        );
        assert_eq!(multi_select.multi_select.get(1).unwrap().name, "CSS");
        assert_eq!(
            multi_select.multi_select.get(1).unwrap().color,
            Some(crate::others::select::SelectColor::Blue)
        );
    }
}
