#![no_std]
#![no_main]

use bootloader_api::entry_point;
use bootloader_x86_64_common::framebuffer;
use core::panic::PanicInfo;
mod writer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(buffer) = boot_info.framebuffer.as_mut() {
        let info = buffer.info();
        let writer = framebuffer::FrameBufferWriter::new(buffer.buffer_mut(), info);
        *writer::WRITER.lock() = Some(writer);
    }
    println!("abc");
    println!("123");
    loop {}
}
