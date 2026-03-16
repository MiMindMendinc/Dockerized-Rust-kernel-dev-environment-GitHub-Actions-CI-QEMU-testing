#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

/// Kernel entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}

/// Panic handler for non-test builds.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Panic handler for test builds.
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Custom test runner – iterates over all `#[test_case]` functions.
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
