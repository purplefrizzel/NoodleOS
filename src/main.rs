#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(noodleos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use noodleos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    noodleos::init();

    println!("Hello world!");

    #[cfg(test)]
    test_main();

    noodleos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    noodleos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    noodleos::test_panic_handler(info)
}
