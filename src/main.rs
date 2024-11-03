#![no_std] // disables standard library
#![no_main] // overwrites the normal entry point

mod vga_buffer; // rust module to handle printing

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

// overwriting os entry point with our own func
#[no_mangle] // disables name mangling to ensure func name is _start
pub extern "C" fn _start() -> ! {
  // extern "C" to use C calling convention
  // this func is the entry point, since the linker looks for a func
  // named '_start' by default
  
  use core::fmt::Write;
  write!(vga_buffer::WRITER.lock(), "ferrOS version {}",0.1/10.0).unwrap();
  vga_buffer::WRITER.lock().write_str("

        ______
       (  =)  )
    ___________
       dGGGGMMb     ,................
      @p~qp~~qMb    | Enlightenment |
      M (@)(@) M   _;...............'
      @,----.JM| -'
     JS^|__/  qKL
    dZP        qKRb
   dZP          qKKb
  fZP            SMMb
  HZM            MMMM
  FqM            MMMM
 _qM.         ..MqML
/./  `.       | `' ..
'.     |____.,|     .'
  '.   )MMMMMM|   .'
    `-'       `--'

");

  loop {}
}
