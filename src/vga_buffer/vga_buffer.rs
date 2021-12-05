use core::fmt::Write;
use volatile::Volatile;
use crate::vga_buffer::color::{ColorCode, NORMAL_COLOR};
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BLANK_CHAR: ScreenChar = ScreenChar {
    ascii_char: b' ',
    color_code: NORMAL_COLOR,
};


const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VgaBuffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

impl VgaBuffer {
    fn screen_vga_buffer() -> &'static mut VgaBuffer {
        unsafe { &mut *(0xb8000 as *mut VgaBuffer) }
    }
}


pub struct VgaWriter {
    next_column_position: usize,
    next_row_position: usize,
    buffer: &'static mut VgaBuffer,
}

impl VgaWriter {
    fn write_string(&mut self, str: &str) {
        self.write_string_with_color(str, NORMAL_COLOR)
    }

    fn write_string_with_color(&mut self, str: &str, color: ColorCode) {
        let default_unknown_char = 0xfe as u8;
        for byte in str.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.write_ascii_byte(byte, color),
                // 不包含在上述范围之内的字节
                _ => self.write_ascii_byte(default_unknown_char, color),
            }
        }
    }

    fn write_ascii_byte(&mut self, byte: u8, color: ColorCode) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.next_column_position >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[self.next_row_position][self.next_column_position].write(ScreenChar {
                    ascii_char: byte,
                    color_code: color,
                });

                self.next_column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        if self.next_row_position == VGA_BUFFER_HEIGHT - 1 {
            for row in 1..VGA_BUFFER_HEIGHT {
                for col in 0..VGA_BUFFER_WIDTH {
                    let char = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(char);
                }
            }

            for col in 0..VGA_BUFFER_WIDTH {
                self.buffer.chars[VGA_BUFFER_HEIGHT - 1][col].write(BLANK_CHAR);
            }
        }


        self.next_column_position = 0;

        if self.next_row_position < VGA_BUFFER_HEIGHT - 1 {
            self.next_row_position += 1;
        }
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
pub static ref DEFAULT_WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
            next_row_position: 0,
            next_column_position: 0,
            buffer: VgaBuffer::screen_vga_buffer(),
        });
}