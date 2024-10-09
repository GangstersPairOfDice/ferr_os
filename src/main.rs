#![no_std] // disables standard library
#![no_main] // overwrites the normal entry point

// since no std_lib
// need to write own panic handler func
use core::panic::PanicInfo;

// this func is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  // PanicInfo contains file and line where panic happened
  // and optional panic message
  // ! = never return type
  loop {}
}

static HELLO: &[u8] = b"Hello World!";

// overwriting os entry point with our own func
#[no_mangle] // disables name mangling to ensure func name is _start
pub extern "C" fn _start() -> ! {
  // extern "C" to use C calling convention
  // this func is the entry point, since the linker looks for a func
  // named '_start' by default
  
  let vga_buffer = 0xb8000 as *mut u8; // location of vga buffer

  for (i, &byte) in HELLO.iter().enumerate() {
  // iters through bytes of HELLO byte string
  // enumerate() to get a running variable i

  // tells Rust we are sure the ops are valid
    unsafe {
      // offset method to write the string byte & color  
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // light cyan
    }
  }

  loop {}
}
