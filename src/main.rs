#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use x86_64::instructions::port::Port;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    serial_println("Kernel started");
    qemu_exit_success();
}

/// Write a single byte to the first serial port (COM1, 0x3F8).
///
/// QEMU emulates 16550 UART; no initialisation is required for simple
/// output in the emulator.
fn serial_write_byte(byte: u8) {
    let mut port: Port<u8> = Port::new(0x3F8);
    // Safety: writing to the COM1 data register is always safe in this
    // bare-metal context.
    unsafe { port.write(byte) };
}

fn serial_println(s: &str) {
    for byte in s.bytes() {
        serial_write_byte(byte);
    }
    serial_write_byte(b'\r');
    serial_write_byte(b'\n');
}

/// Signal QEMU to exit via the `isa-debug-exit` device.
///
/// QEMU is started with `-device isa-debug-exit,iobase=0xf4,iosize=0x04`.
/// Writing value `v` to port 0xF4 causes QEMU to exit with code
/// `(v << 1) | 1`.  Value 0x10 → exit code 33 (matches test-success-exit-code
/// in Cargo.toml).
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
