use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#relation>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"relation"`
/// - `$.['*'].relation`: An array of related page references.
///                       A page reference is an object with an id key and
///                       a string value corresponding to a page ID in another database.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example relation page property value
///
/// ```json
/// {
///   "Related": {
///     "id": "b%7D%3Ek",
///     "type": "relation",
///     "relation": [
///       {
///         "id": "669ffc58-9c20-4264-956b-f7f917c58400"
///       }
///     ],
///     "has_more": false
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRelationProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: Option<String>,

    /// An array of related page references.
    /// A page reference is an object with an id key and
    /// a string value corresponding to a page ID in another database.
    pub relation: Vec<PageRelationPropertyParameter>,

    /// If a relation has more than 25 references,
    /// then the has_more value for the relation in the response object is true.
    /// If a relation doesn’t exceed the limit, then has_more is false.
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRelationPropertyParameter {
    /// related page id
    pub id: String,
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
    fn deserialize_relation_property() {
        let json_data = r#"
        {
            "Related": {
                "id": "b%7D%3Ek",
                "type": "relation",
                "relation": [
                    {
                        "id": "669ffc58-9c20-4264-956b-f7f917c58400"
                    }
                ],
                "has_more": false
            }
        }
        "#;

        let relation_map = serde_json::from_str::<
            std::collections::HashMap<String, PageRelationProperty>,
        >(json_data)
        .unwrap();

        let relation = relation_map.get("Related").unwrap();

        assert_eq!(relation.id, Some("b%7D%3Ek".to_string()));
        assert!(!relation.has_more);

        assert_eq!(
            relation.relation.first().unwrap().id,
            "669ffc58-9c20-4264-956b-f7f917c58400"
        );
    }
}
