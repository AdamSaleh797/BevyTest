use bevy::color::Color;
use strum::{EnumIter, EnumTable};

#[derive(EnumTable, EnumIter, Clone, Copy)]
pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}
impl PrimaryColor {
    pub fn to_bevy_color(self) -> Color {
        match self {
            Self::Red => Color::srgb_u8(244, 56, 1),
            Self::Yellow => Color::srgb_u8(240, 196, 1),
            Self::Blue => Color::srgb_u8(44, 104, 187),
        }
    }

    pub fn blend(self, other: Self) -> Palette {
        match (self, other) {
            (Self::Red, Self::Red) => Palette::Red,
            (Self::Yellow, Self::Yellow) => Palette::Yellow,
            (Self::Blue, Self::Blue) => Palette::Blue,
            (Self::Yellow, Self::Blue) | (Self::Blue, Self::Yellow) => Palette::Green,
            (Self::Red, Self::Yellow) | (Self::Yellow, Self::Red) => Palette::Orange,
            (Self::Red, Self::Blue) | (Self::Blue, Self::Red) => Palette::Purple,
        }
    }
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Debug)]
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

    pub fn constituents(self) -> (PrimaryColor, PrimaryColor) {
        match self {
            Self::Red => (PrimaryColor::Red, PrimaryColor::Red),
            Self::Yellow => (PrimaryColor::Yellow, PrimaryColor::Yellow),
            Self::Blue => (PrimaryColor::Blue, PrimaryColor::Blue),
            Self::Green => (PrimaryColor::Yellow, PrimaryColor::Blue),
            Self::Orange => (PrimaryColor::Red, PrimaryColor::Yellow),
            Self::Purple => (PrimaryColor::Red, PrimaryColor::Blue),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::palette::Palette;

    #[test]
    fn test_constituents() {
        for color in Palette::iter() {
            let (c1, c2) = color.constituents();
            assert_eq!(c1.blend(c2), color);
            assert_eq!(c2.blend(c1), color);
        }
    }
}
