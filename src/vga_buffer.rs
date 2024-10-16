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
  fn new(foreground: Color, background: Color) -> ColorCode {
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
  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // printable ASCII byte or newline
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // not part of printable ASCII range
        _ => self.write_byte(0xfe), // prints a ■ char for unprintable bytes
      }
    }
  }
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT -1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col] = ScreenChar {
          ascii_character: byte,
          color_code,
        };
        self.column_position +=1;
      }
    }
  }

  fn new_line(&mut self) {/* TODO */}
}

pub fn splash_screen() {
  let mut writer = Writer {
     column_position: 0,
     color_code: ColorCode::new(Color::Yellow, Color::Black),
     buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // points to VGA buffer
   };

   writer.write_string("
        ______\n
       (  =)  )\n
    ___________\n
       dGGGGMMb     ,................\n
      @p~qp~~qMb    | Enlightenment |\n
      M|@||@) M|   _;...............'\n
      @,----.JM| -'\n
     JS^|__/  qKL\n
    dZP        qKRb\n
   dZP          qKKb\n
  fZP            SMMb\n
  HZM            MMMM\n
  FqM            MMMM\n
 _qM.         ..MqML\n
/./  `.       | `' ..\n
'.     |____.,|     .'\n
  '.   )MMMMMM|   .'\n
    `-'       `--'\n
\n
")

}
