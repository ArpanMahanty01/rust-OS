#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate os;

use os::{ vga_buffer};

// static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info:&PanicInfo)->!{
loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("\n will this be in new line \n and this?").unwrap();

    // write!(vga_buffer::WRITER.lock(),"this works {},{}",42,34.11).unwrap();
    // write!(vga_buffer::WRITER.lock(),"\n will this be in new line \n and this?").unwrap();

    // println!("Hello World{}", "!");

    os::init();
    // x86_64::instructions::interrupts::int3();
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }
    // vga_buffer::WRITER.lock().write_str("\n this did not broke").unwrap();

    fn stack_overflow(){
        stack_overflow();
    }

    stack_overflow();

    loop {}
}


