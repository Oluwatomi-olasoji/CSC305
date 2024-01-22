#![no_std]
#![no_main] //we dont use a main function in this file
use x86_64::instructions::{hlt};

static HELLO: &[u8] = b"Hello World! This is 
just a quick illustration";


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
 loop {hlt();}
}

/*#[no_mangle]
pub extern "C" fn _start() -> ! {
 loop {}
}*/
#[no_mangle]
pub extern "C" fn _start() -> ! {
 let framebuffer = 0xb8000 as *mut u8;
 /*unsafe {
 framebuffer.offset(1).write_volatile(0x30);
 }*/
 for (i, &byte) in HELLO.iter().enumerate() {
 unsafe {
 *framebuffer.offset(i as isize * 2) = byte;
 *framebuffer.offset(i as isize * 2 + 1) = 0xb;
 }
 }
 loop {hlt();} //this loop means the OS is running forever
}
