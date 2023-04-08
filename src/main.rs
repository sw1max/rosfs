#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osrs::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osrs::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    osrs::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    osrs::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    osrs::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osrs::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
