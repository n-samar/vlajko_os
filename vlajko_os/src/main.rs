#![no_std]
#![no_main]

mod vga_buffer;

static HELLO: &[u8] = b"Srbi s Bembarama! Upozorenje: Nije namenjeno Srbima bez Bembara";

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Some message");

    loop {}
}
