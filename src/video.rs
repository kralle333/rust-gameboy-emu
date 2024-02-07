use sdl2::pixels::Color;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const PIXEL_SIZE: usize = 4;

#[allow(dead_code)]
pub enum ColorScheme {
    Green,
    BlackWhite,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GBColor {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

pub fn byte_to_color(val: u8) -> GBColor {
    match val {
        0 => GBColor::White,
        1 => GBColor::LightGray,
        2 => GBColor::DarkGray,
        3 => GBColor::Black,
        _ => panic!("what"),
    }
}

pub fn get_color(color: &GBColor, scheme: &ColorScheme) -> Color {
    match scheme {
        ColorScheme::Green => match color {
            GBColor::White => Color::RGBA(0x9C, 0xBD, 0x0F, 0xFF),
            GBColor::LightGray => Color::RGBA(0x8C, 0xAD, 0x0F, 0xFF),
            GBColor::DarkGray => Color::RGBA(0x30, 0x62, 0x30, 0xFF),
            GBColor::Black => Color::RGBA(0x0F, 0x38, 0x0F, 0xFF),
        },
        ColorScheme::BlackWhite => match color {
            GBColor::White => Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF),
            GBColor::LightGray => Color::RGBA(0x8C, 0x8C, 0x8C, 0xFF),
            GBColor::DarkGray => Color::RGBA(0x30, 0x30, 0x30, 0xFF),
            GBColor::Black => Color::RGBA(0, 0, 0, 0xFF),
        },
    }
}
