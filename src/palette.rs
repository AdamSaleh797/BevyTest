use bevy::color::Color;

use crate::color_mix::color::{self, PrimaryColor};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Palette {
    Red,
    Yellow,
    Blue,
    Green,
    Orange,
    Purple,
}
impl Palette {
    pub fn to_bevy_color(self) -> Color {
        match self {
            Palette::Red => Color::srgb_u8(244, 56, 1),
            Palette::Yellow => Color::srgb_u8(240, 196, 1),
            Palette::Blue => Color::srgb_u8(44, 104, 187),
            Palette::Green => Color::srgb_u8(99, 133, 25),
            Palette::Orange => Color::srgb_u8(244, 127, 1),
            Palette::Purple => Color::srgb_u8(200, 116, 217),
        }
    }
}

impl From<PrimaryColor> for Palette {
    fn from(value: PrimaryColor) -> Self {
        match value {
            PrimaryColor::Red => Self::Red,
            PrimaryColor::Yellow => Self::Yellow,
            PrimaryColor::Blue => Self::Blue,
        }
    }
}

impl From<color::Color> for Palette {
    fn from(value: color::Color) -> Self {
        match value {
            color::Color::Red => Self::Red,
            color::Color::Yellow => Self::Yellow,
            color::Color::Blue => Self::Blue,
            color::Color::Green => Self::Green,
            color::Color::Orange => Self::Orange,
            color::Color::Purple => Self::Purple,
        }
    }
}
