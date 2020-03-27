#![no_std]
#![no_main]

use core::{
    mem,
    fmt::Write,
    panic::PanicInfo,
};

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::WRITER.lock();
    writeln!(writer, "Hello").unwrap();
    writeln!(writer, "World").unwrap();
    mem::drop(writer);
    println!("Hello");
    print!("World");

    loop {}
}
