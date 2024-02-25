mod vga_buffer;

#![no_std] // Do not link standard library
#![no_main]


use core::panic::PanicInfo;

static HELLO: &[u8] = b"Helloooooo World!";

/// This is the entrypoint for the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    vga_buffer::print_something();

    loop {}
}

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

