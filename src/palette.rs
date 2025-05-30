use bevy::color::Color;

pub enum Palette {
    Red,
    Yellow,
    Blue,
    Green,
    Orange,
    Purple,
}
impl Palette {
    pub fn to_bevy_color(&self) -> Color {
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
