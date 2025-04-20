use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        // Bit scheme of the VGA attribute
        // 7654 3210
        // 0000 0000
        //  fg   bg
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
struct ScreenChar {
    char: u8,
    attribute: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
#[repr(transparent)]
struct Buffer {
    // Volatile prevents compiler optimization of RAM writes
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref STDOUT: Mutex<Writer> = Mutex::new(Writer {
        column_pos: 0,
        color: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static! {
    pub static ref STDERR: Mutex<Writer> = Mutex::new(Writer {
        column_pos: 0,
        color: ColorCode::new(Color::LightRed, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                self.buffer.chars[row][col].write(ScreenChar {
                    char: byte,
                    attribute: self.color,
                });

                self.column_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            char: b' ',
            attribute: ColorCode::new(Color::Black, Color::Black),
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(()) // Writes to VGA never fail
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    STDOUT.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    STDERR.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga_buffer::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}
