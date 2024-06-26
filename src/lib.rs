#![feature(abi_x86_interrupt)]
#![no_std]
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}
