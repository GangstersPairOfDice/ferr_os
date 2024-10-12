#[allow(dead_code)] // disables warning for unused Color vars
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // derives some basic op implementations
#[repr(u8)] // stores color vars as u8
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // ensures same data layout as u8
struct ColorCode(u8); // contains full color byte

impl ColorCode {
  fn new(foreground: Color, Background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}
