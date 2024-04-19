use crate::vga_buffer;
use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}
pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame:InterruptStackFrame){
    use core::fmt::Write;
    write!(vga_buffer::WRITER.lock(),"EXCEPTION: BREAKPOINT\n{:#?}",stack_frame).unwrap();
}