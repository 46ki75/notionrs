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
