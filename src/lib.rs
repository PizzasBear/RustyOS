#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;

pub mod vga_buff;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop {}
}

#[no_mangle]
pub fn rust_main() -> ! {
    for i in 0..3 {
        println!("Hello World! {}", i);
    }
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
// #[lang = "panic_fmt"]
// #[no_mangle]
// pub extern "C" fn panic_fmt() -> ! {
//     loop {}
// }
