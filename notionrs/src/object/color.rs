use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Default,
    Blue,
    Brown,
    Gray,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,

    DefaultBackground,
    BlueBackground,
    BrownBackground,
    GrayBackground,
    GreenBackground,
    OrangeBackground,
    PinkBackground,
    PurpleBackground,
    RedBackground,
    YellowBackground,
}

impl std::str::FromStr for Color {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str(s)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_plain::to_string(self)
            .map_err(|_| std::fmt::Error)?
            .fmt(f)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use std::str::FromStr;

    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_color_fg() {
        let json_data = json!("blue");

        let color: Color = serde_json::from_value(json_data).unwrap();

        assert_eq!(color, Color::Blue);
    }

    #[test]
    fn deserialize_color_bg() {
        let json_data = json!("blue_background");

        let color: Color = serde_json::from_value(json_data).unwrap();

        assert_eq!(color, Color::BlueBackground);
    }

    #[test]
    fn deserialize_invalid_color() {
        let json_data = json!("invalid_color");

        let result: Result<Color, _> = serde_json::from_value(json_data);
        assert!(result.is_err());
    }

    #[test]
    fn check_trait_display() {
        let blue_background: Color = Color::BlueBackground;
        assert_eq!(blue_background.to_string(), "blue_background".to_string());
    }

    #[test]
    fn check_trait_try_from() {
        let color: Color = Color::from_str("blue").unwrap();
        assert_eq!(color, Color::Blue);
    }

    #[test]
    fn check_from_str_color() {
        let cases = vec![
            ("blue", Some(Color::Blue)),
            ("brown", Some(Color::Brown)),
            ("gray", Some(Color::Gray)),
            ("green", Some(Color::Green)),
            ("orange", Some(Color::Orange)),
            ("pink", Some(Color::Pink)),
            ("purple", Some(Color::Purple)),
            ("red", Some(Color::Red)),
            ("yellow", Some(Color::Yellow)),
            ("blue_background", Some(Color::BlueBackground)),
            ("brown_background", Some(Color::BrownBackground)),
            ("gray_background", Some(Color::GrayBackground)),
            ("green_background", Some(Color::GreenBackground)),
            ("orange_background", Some(Color::OrangeBackground)),
            ("pink_background", Some(Color::PinkBackground)),
            ("purple_background", Some(Color::PurpleBackground)),
            ("red_background", Some(Color::RedBackground)),
            ("yellow_background", Some(Color::YellowBackground)),
            ("default", Some(Color::Default)),
            ("invalid_color", None),
        ];

        for (input, expected) in cases {
            match Color::from_str(input) {
                Ok(color) => assert_eq!(Some(color), expected, "Input: {}", input),
                Err(_) => assert!(expected.is_none(), "Expected None for input: {}", input),
            }
        }
    }
}
