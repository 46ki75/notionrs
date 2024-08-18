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
// macro
//
// # --------------------------------------------------------------------------------

#[macro_export]
macro_rules! color_setters {
    ($self:ident, $color_target:expr) => {
        pub fn default_color(mut $self) -> Self {
            $color_target = $crate::others::color::Color::default();
            $self
        }
        pub fn blue(mut $self) -> Self {
            $color_target = $crate::others::color::Color::blue();
            $self
        }
        pub fn brown(mut $self) -> Self {
            $color_target = $crate::others::color::Color::brown();
            $self
        }
        pub fn gray(mut $self) -> Self {
            $color_target = $crate::others::color::Color::gray();
            $self
        }
        pub fn green(mut $self) -> Self {
            $color_target = $crate::others::color::Color::green();
            $self
        }
        pub fn orange(mut $self) -> Self {
            $color_target = $crate::others::color::Color::orange();
            $self
        }
        pub fn pink(mut $self) -> Self {
            $color_target = $crate::others::color::Color::pink();
            $self
        }
        pub fn purple(mut $self) -> Self {
            $color_target = $crate::others::color::Color::purple();
            $self
        }
        pub fn red(mut $self) -> Self {
            $color_target = $crate::others::color::Color::red();
            $self
        }
        pub fn yellow(mut $self) -> Self {
            $color_target = $crate::others::color::Color::yellow();
            $self
        }
        pub fn blue_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::blue_background();
            $self
        }
        pub fn brown_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::brown_background();
            $self
        }
        pub fn gray_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::gray_background();
            $self
        }
        pub fn green_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::green_background();
            $self
        }
        pub fn orange_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::orange_background();
            $self
        }
        pub fn pink_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::pink_background();
            $self
        }
        pub fn purple_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::purple_background();
            $self
        }
        pub fn red_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::red_background();
            $self
        }
        pub fn yellow_background(mut $self) -> Self {
            $color_target = $crate::others::color::Color::yellow_background();
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
