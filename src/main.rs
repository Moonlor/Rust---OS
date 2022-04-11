#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
// The function should never return, so it is marked as a diverging function by returning the “never” type !
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// By using the #[no_mangle] attribute we disable the name mangling to ensure that the Rust compiler really outputs a function with the name _start.
// Mark the function as extern "C" to tell the compiler that it should use the C calling convention for this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

