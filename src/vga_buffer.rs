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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // lays out struct fields like a C struct, ensures correct field ordering
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] // again, ensure same memory layout as its single field
struct Buffer {
  chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
  column_position: usize, // keeps track of current position in latest row
  color_code: ColorCode, // fore&back-ground colors
  buffer: &'static mut Buffer, // ref to VGA buffer
  // 'static lifetime tells compiles the reference is valid for the whole program runtime
}

impl Writer {
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT -1;
        let col = self.column.position;

        let color_code = self.color_code;
        self.buffer.chars[row][col] = ScreenChar {
          ascii_character: byte,
          color_code,
        };
        self.column_posiiton +=1;
      }
    }
  }

  fn new_line(&mut self) {/* TODO */}
}
