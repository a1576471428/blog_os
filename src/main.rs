#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello World!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn print(str: &str) {
    let vga_buffer_start = 0xb8000 as *mut u8;
    for (index, byte) in str.as_bytes().iter().enumerate() {
        unsafe {
            // content
            *vga_buffer_start.offset(index as isize * 2) = *byte;
            // color
            *vga_buffer_start.offset(index as isize * 2 + 1) = 0xb;
        }
    }
}