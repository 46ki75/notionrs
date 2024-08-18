use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColorFG {
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
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColorBG {
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(untagged)]
pub enum Color {
    FG(ColorFG),
    BG(ColorBG),
}

impl Default for Color {
    fn default() -> Self {
        Color::FG(ColorFG::Default)
    }
}

impl Color {
    pub fn blue() -> Self {
        Color::FG(ColorFG::Blue)
    }

    pub fn brown() -> Self {
        Color::FG(ColorFG::Brown)
    }

    pub fn gray() -> Self {
        Color::FG(ColorFG::Gray)
    }

    pub fn green() -> Self {
        Color::FG(ColorFG::Green)
    }

    pub fn orange() -> Self {
        Color::FG(ColorFG::Orange)
    }

    pub fn pink() -> Self {
        Color::FG(ColorFG::Pink)
    }

    pub fn purple() -> Self {
        Color::FG(ColorFG::Purple)
    }

    pub fn red() -> Self {
        Color::FG(ColorFG::Red)
    }

    pub fn yellow() -> Self {
        Color::FG(ColorFG::Yellow)
    }

    pub fn blue_background() -> Self {
        Color::BG(ColorBG::BlueBackground)
    }

    pub fn brown_background() -> Self {
        Color::BG(ColorBG::BrownBackground)
    }

    pub fn gray_background() -> Self {
        Color::BG(ColorBG::GrayBackground)
    }

    pub fn green_background() -> Self {
        Color::BG(ColorBG::GreenBackground)
    }

    pub fn orange_background() -> Self {
        Color::BG(ColorBG::OrangeBackground)
    }

    pub fn pink_background() -> Self {
        Color::BG(ColorBG::PinkBackground)
    }

    pub fn purple_background() -> Self {
        Color::BG(ColorBG::PurpleBackground)
    }

    pub fn red_background() -> Self {
        Color::BG(ColorBG::RedBackground)
    }

    pub fn yellow_background() -> Self {
        Color::BG(ColorBG::YellowBackground)
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
    use serde_json::json;

    #[test]
    fn deserialize_color_fg() {
        let json_data = json!("blue");

        let color: Color = serde_json::from_value(json_data).unwrap();

        assert_eq!(color, Color::FG(ColorFG::Blue));
    }

    #[test]
    fn deserialize_color_bg() {
        let json_data = json!("blue_background");

        let color: Color = serde_json::from_value(json_data).unwrap();

        assert_eq!(color, Color::BG(ColorBG::BlueBackground));
    }

    #[test]
    fn deserialize_invalid_color() {
        let json_data = json!("invalid_color");

        let result: Result<Color, _> = serde_json::from_value(json_data);
        assert!(result.is_err());
    }
}
