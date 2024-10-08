#![no_std] // disables standard library

// since no std_lib
// need to write own panic handler func
use core::panic::PanicInfo;

// this func is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
// PaniInfo contains file and line where panic happened
// and optional panic message
// ! = never return type

fn main() {}
