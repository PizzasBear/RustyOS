#![no_std]
#![feature(ptr_internals)]
#![feature(lang_items)]

extern crate spin;
extern crate volatile;
use core::panic::PanicInfo;

pub mod vga_buff;

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear!();
    for i in 0..3 {
        println!("Hello World! {}", i);
    }
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
