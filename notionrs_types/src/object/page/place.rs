use serde::{Deserialize, Serialize};

/// Notion API Reference is not yet available for this property,
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"place"`
/// - `$.['*'].place`: An array of objects containing information about
///   the [places](https://developers.notion.com/reference/place-object).
///   If the place does not exist, an empty array will be returned.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example files page property value
///
/// ```json
/// {
///     "Place": {
///         "id": "%60j%3Bh",
///         "type": "place",
///         "place": {
///             "lat": 35.7357,
///             "lon": 139.65161,
///             "name": "Nerima-ku, Tokyo, Japan",
///             "address": "Nerima-ku, Tokyo, Japan",
///             "aws_place_id": "AQAAADkAj5aGFJcsAOdCoyrWV09Cn_AGAFSpLo37jXNvIb3dUlWND7nWItPkOH_InLtxQOYctjf-J_sLJxEz18aFXjUF4dbw3HNxwMmNucyPj74WUhp5hlF7qt93f3Y",
///             "google_place_id": null
///         }
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PagePlaceProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An array of objects containing information
    /// about the [files](https://developers.notion.com/reference/file-object).
    ///
    /// When creating, both the external path of the file and `name` are required.
    pub place: Option<PagePlacePropertyParameter>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PagePlacePropertyParameter {
    /// Latitude
    pub lat: f64,

    /// Longitude
    pub lon: f64,

    /// Name of the place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Address of the place
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// AWS Place ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_place_id: Option<String>,

    /// Google Place ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
}

impl std::fmt::Display for PagePlaceProperty {
    /// display the files' names in a comma-separated list
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.place
                .as_ref()
                .map(|p| p.name.clone().unwrap_or_default())
                .unwrap_or_default()
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
    fn deserialize_page_files_property() {
        let json_data = r#"
        {
            "Place": {
                "id": "%60j%3Bh",
                "type": "place",
                "place": {
                    "lat": 35.7357,
                    "lon": 139.65161,
                    "name": "Nerima-ku, Tokyo, Japan",
                    "address": "Nerima-ku, Tokyo, Japan",
                    "aws_place_id": "AQAAADkAj5aGFJcsAOdCoyrWV09Cn_AGAFSpLo37jXNvIb3dUlWND7nWItPkOH_InLtxQOYctjf-J_sLJxEz18aFXjUF4dbw3HNxwMmNucyPj74WUhp5hlF7qt93f3Y",
                    "google_place_id": null
                }
            }
        }
        "#;

        let place_map =
            serde_json::from_str::<std::collections::HashMap<String, PagePlaceProperty>>(json_data)
                .unwrap();

        let place = place_map.get("Place").unwrap();

        assert_eq!(place.id, Some("%60j%3Bh".to_string()));
        let p = place.place.as_ref().unwrap();
        assert_eq!(p.lat, 35.7357);
        assert_eq!(p.lon, 139.65161);
        assert_eq!(p.name.as_ref().unwrap(), "Nerima-ku, Tokyo, Japan");
        assert_eq!(p.address.as_ref().unwrap(), "Nerima-ku, Tokyo, Japan");
        assert_eq!(
            p.aws_place_id.as_ref().unwrap(),
            "AQAAADkAj5aGFJcsAOdCoyrWV09Cn_AGAFSpLo37jXNvIb3dUlWND7nWItPkOH_InLtxQOYctjf-J_sLJxEz18aFXjUF4dbw3HNxwMmNucyPj74WUhp5hlF7qt93f3Y"
        );
        assert_eq!(p.google_place_id, None);
    }
}
