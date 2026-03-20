#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u16;
    let hello = b"Hello from Michigan MindMend Rust Kernel!  \xf0\x9f\x9a\x80\xef\xb8\x8f";

    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.add(i) = (byte as u16) | (0x0f << 8);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}.