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
#[derive(Debug, Deserialize, Serialize)]
pub struct PageMultiSelectProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// Array of Select object
    pub multi_select: Vec<crate::others::select::Select>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_deserialize_page_multi_select_property() {
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

        assert_eq!(multi_select.id, "_bnY");

        assert_eq!(
            multi_select.multi_select.first().unwrap().id,
            "959ba3e3-5a64-4ee6-864b-9e94ddc024d5"
        );
        assert_eq!(multi_select.multi_select.first().unwrap().name, "HTML");
        assert_eq!(
            multi_select.multi_select.first().unwrap().color,
            crate::others::color::ColorFG::Orange
        );

        assert_eq!(
            multi_select.multi_select.get(1).unwrap().id,
            "f22b05c9-0225-4dee-b25b-db7e63a47e0b"
        );
        assert_eq!(multi_select.multi_select.get(1).unwrap().name, "CSS");
        assert_eq!(
            multi_select.multi_select.get(1).unwrap().color,
            crate::others::color::ColorFG::Blue
        );
    }
}
