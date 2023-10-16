use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_position: 0,
        column_position: 0,
        attribute: VgaTextAttribute::new(Color::Blue, true, Color::Black, false),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Writer {
    row_position: usize,
    column_position: usize,
    attribute: VgaTextAttribute,
    buffer: &'static mut Buffer,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<VgaText>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaText {
    character: u8,
    attribute: VgaTextAttribute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct VgaTextAttribute(u8);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
}

impl VgaTextAttribute {
    fn new(
        foreground_color: Color,
        foreground_color_brightness: bool,
        background_color: Color,
        blink: bool,
    ) -> VgaTextAttribute {
        VgaTextAttribute(
            (foreground_color as u8)
                | if foreground_color_brightness { 1 } else { 0 } << 3
                | (background_color as u8) << 4
                | if blink { 1 } else { 0 } << 7,
        )
    }
}

impl Writer {
    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position = self.row_position + 1;
        } else {
            for i in 1..BUFFER_HEIGHT {
                for j in 0..BUFFER_WIDTH {
                    self.buffer.chars[i - 1][j].write(self.buffer.chars[i][j].read());
                    self.clear_row(BUFFER_HEIGHT - 1);
                }
            }
        }
        self.column_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = VgaText {
            character: b' ',
            attribute: self.attribute,
        };
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[row][i].write(blank);
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            }
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buffer.chars[self.row_position][self.column_position].write(VgaText {
                    character: byte,
                    attribute: self.attribute,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
