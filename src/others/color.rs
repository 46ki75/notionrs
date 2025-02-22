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
        write!(f, "{}", serde_plain::to_string(self).unwrap())
    }
}

// # --------------------------------------------------------------------------------
//
// macro
//
// # --------------------------------------------------------------------------------

#[macro_export]
macro_rules! color_setters {
    ($self:ident, $color_target:expr) => {
        pub fn default_color(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Default;
            $self
        }
        pub fn blue(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Blue;
            $self
        }
        pub fn brown(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Brown;
            $self
        }
        pub fn gray(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Gray;
            $self
        }
        pub fn green(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Green;
            $self
        }
        pub fn orange(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Orange;
            $self
        }
        pub fn pink(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Pink;
            $self
        }
        pub fn purple(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Purple;
            $self
        }
        pub fn red(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Red;
            $self
        }
        pub fn yellow(mut $self) -> Self {
            $color_target = $crate::others::color::Color::Yellow;
            $self
        }
        pub fn default_background_color(mut $self) -> Self {
            $color_target = $crate::others::color::Color::DefaultBackground;
            $self
        }
        pub fn blue_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::BlueBackground;
            $self
        }
        pub fn brown_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::BrownBackground;
            $self
        }
        pub fn gray_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::GrayBackground;
            $self
        }
        pub fn green_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::GreenBackground;
            $self
        }
        pub fn orange_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::OrangeBackground;
            $self
        }
        pub fn pink_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::PinkBackground;
            $self
        }
        pub fn purple_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::PurpleBackground;
            $self
        }
        pub fn red_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::RedBackground;
            $self
        }
        pub fn yellow_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::YellowBackground;
            $self
        }
    };
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
