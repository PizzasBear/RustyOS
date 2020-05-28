// #![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate bootloader;

use core::panic::PanicInfo;

pub mod vga_buff;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop {}
}

#[no_mangle]
pub fn _start() -> ! {
    for i in 0..3 {
        println!("Hello World! {}", i);
    }
    loop {}
}
