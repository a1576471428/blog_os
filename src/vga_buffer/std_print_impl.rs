use core::fmt;
use crate::vga_buffer::vga_buffer::DEFAULT_WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::std_print_impl::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    DEFAULT_WRITER.lock().write_fmt(args).unwrap();
}