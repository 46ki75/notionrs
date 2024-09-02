use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseNumberProperty {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub number: DatabaseNumberFormatProperty,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct DatabaseNumberFormatProperty {
    pub format: NumberFormat,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
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

        assert_eq!(number.id, "%7B%5D_P");
        assert_eq!(number.name, "Price");
        assert_eq!(number.number.format, NumberFormat::Dollar);
    }
}
