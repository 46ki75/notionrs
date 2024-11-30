use serde::{Deserialize, Serialize};

#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default, strum_macros::Display,
)]
#[strum(serialize_all = "snake_case")]
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

impl<T> From<T> for Color
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref() {
            "default" => Color::Default,
            "blue" => Color::Blue,
            "brown" => Color::Brown,
            "gray" => Color::Gray,
            "green" => Color::Green,
            "orange" => Color::Orange,
            "pink" => Color::Pink,
            "purple" => Color::Purple,
            "red" => Color::Red,
            "yellow" => Color::Yellow,
            "blue_background" => Color::BlueBackground,
            "brown_background" => Color::BrownBackground,
            "gray_background" => Color::GrayBackground,
            "green_background" => Color::GreenBackground,
            "orange_background" => Color::OrangeBackground,
            "pink_background" => Color::PinkBackground,
            "purple_background" => Color::PurpleBackground,
            "red_background" => Color::RedBackground,
            "yellow_background" => Color::YellowBackground,
            _ => Color::Default,
        }
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
    fn check_trait_from() {
        let blue = Color::from("blue");
        assert_eq!(blue, Color::Blue);
    }

    #[test]
    fn check_trait_display() {
        let blue_background: Color = Color::BlueBackground;
        assert_eq!(blue_background.to_string(), "blue_background".to_string());
    }
}
