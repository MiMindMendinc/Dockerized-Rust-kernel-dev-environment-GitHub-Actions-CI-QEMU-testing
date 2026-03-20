#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use x86_64::instructions::port::Port;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    serial_println("Kernel started");
    qemu_exit_success();
}

fn serial_write_byte(byte: u8) {
    let mut port: Port<u8> = Port::new(0x3F8);
    unsafe { port.write(byte) };
}

fn serial_println(s: &str) {
    for byte in s.bytes() {
        serial_write_byte(byte);
    }
    serial_write_byte(b'\r');
    serial_write_byte(b'\n');
}

fn qemu_exit_success() -> ! {
    let mut exit_port: Port<u32> = Port::new(0xF4);
    unsafe { exit_port.write(0x10u32) };
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println("PANIC occurred");
    let mut exit_port: Port<u32> = Port::new(0xF4);
    unsafe { exit_port.write(0x01u32) };
    loop {
        x86_64::instructions::hlt();
    }
}