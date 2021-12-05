#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ColorCode(u8);

#[allow(dead_code)]
impl ColorCode {
    ///
    /// 前景色和背景色实际上只占用了u4的空间，但是Rust没有u4，所以使用u8类型
    /// 前景色+背景色才是一个u8
    const fn new(foreground: &Color, background: &Color) -> ColorCode {
        ColorCode((*background as u8) << 4 | (*foreground as u8))
    }
}

pub const NORMAL_COLOR: ColorCode = ColorCode::new(&Color::White, &Color::Black);
#[allow(dead_code)]
pub const WARN_COLOR: ColorCode = ColorCode::new(&Color::Yellow, &Color::Black);
#[allow(dead_code)]
pub const ERROR_COLOR: ColorCode = ColorCode::new(&Color::Red, &Color::Black);
#[allow(dead_code)]
pub const HIGHLIGHT_COLOR: ColorCode = ColorCode::new(&Color::LightBlue, &Color::Black);
