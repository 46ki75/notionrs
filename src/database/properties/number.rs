use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseNumberProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub number: DatabaseNumberFormatProperty,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DatabaseNumberFormatProperty {
    pub format: NumberFormat,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
    ArgentinePeso,
    Baht,
    AustralianDollar,
    CanadianDollar,
    ChileanPeso,
    ColombianPeso,
    DanishKrone,
    Dirham,
    Dollar,
    Euro,
    Forint,
    Franc,
    HongKongDollar,
    Koruna,
    Krona,
    Leu,
    Lira,
    MexicanPeso,
    NewTaiwanDollar,
    NewZealandDollar,
    NorwegianKrone,
    #[default]
    Number,
    NumberWithCommas,
    Percent,
    PhilippinePeso,
    Pound,
    PeruvianSol,
    Rand,
    Real,
    Ringgit,
    Riyal,
    Ruble,
    Rupee,
    Rupiah,
    Shekel,
    SingaporeDollar,
    UruguayanPeso,
    Yen,
    Yuan,
    Won,
    Zloty,
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
    fn deserialize_database_number_property() {
        let json_data = r#"
        {
            "id": "%7B%5D_P",
            "name": "Price",
            "type": "number",
            "number": {
                "format": "dollar"
            }
        }
        "#;

        let number = serde_json::from_str::<DatabaseNumberProperty>(json_data).unwrap();

        assert_eq!(number.id, Some("%7B%5D_P".to_string()));
        assert_eq!(number.name, "Price");
        assert_eq!(number.number.format, NumberFormat::Dollar);
    }
}
