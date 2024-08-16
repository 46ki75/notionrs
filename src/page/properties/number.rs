use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#number>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"number"`
/// - `$.['*'].number`: A number representing some value.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example number page property value
///
/// ```json
/// {
///     "Number": {
///         "type": "number",
///         "id": "%5Chme",
///         "number": 20.0
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageNumberProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// A number representing some value.
    pub number: Option<f64>,
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
    fn unit_test_deserialize_number_property() {
        let json_data = r#"
        {
            "Number": {
                "type": "number",
                "id": "%5Chme",
                "number": 20.0
            }
        }
        "#;

        let number_map = serde_json::from_str::<
            std::collections::HashMap<String, PageNumberProperty>,
        >(json_data)
        .unwrap();

        let number = number_map.get("Number").unwrap();

        assert_eq!(number.id, "%5Chme");
        assert_eq!(number.number, Some(20.0));
    }
}
