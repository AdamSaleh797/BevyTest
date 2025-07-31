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
            PrimaryColor::Red => Color::srgb_u8(244, 56, 1),
            PrimaryColor::Yellow => Color::srgb_u8(240, 196, 1),
            PrimaryColor::Blue => Color::srgb_u8(44, 104, 187),
        }
    }

    #[cfg(test)]
    pub fn blend(self, other: Self) -> Palette {
        match (self, other) {
            (PrimaryColor::Red, PrimaryColor::Red) => Palette::Red,
            (PrimaryColor::Yellow, PrimaryColor::Yellow) => Palette::Yellow,
            (PrimaryColor::Blue, PrimaryColor::Blue) => Palette::Blue,
            (PrimaryColor::Yellow, PrimaryColor::Blue)
            | (PrimaryColor::Blue, PrimaryColor::Yellow) => Palette::Green,
            (PrimaryColor::Red, PrimaryColor::Yellow)
            | (PrimaryColor::Yellow, PrimaryColor::Red) => Palette::Orange,
            (PrimaryColor::Red, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Red) => {
                Palette::Purple
            }
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
            Palette::Red => (PrimaryColor::Red, PrimaryColor::Red),
            Palette::Yellow => (PrimaryColor::Yellow, PrimaryColor::Yellow),
            Palette::Blue => (PrimaryColor::Blue, PrimaryColor::Blue),
            Palette::Green => (PrimaryColor::Yellow, PrimaryColor::Blue),
            Palette::Orange => (PrimaryColor::Red, PrimaryColor::Yellow),
            Palette::Purple => (PrimaryColor::Red, PrimaryColor::Blue),
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
