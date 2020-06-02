#![no_std]
#![feature(ptr_internals)]
#![feature(lang_items)]

extern crate multiboot2;
extern crate spin;
extern crate volatile;
use core::panic::PanicInfo;

#[macro_use]
pub mod vga_buff;
use vga_buff::{Color, ColorCode};

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    printc!(ColorCode::new(Color::LightRed, Color::Black));
    println!("\n\n{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    clear!();
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    for i in 0..3 {
        println!("Hello World! {}", i);
    }
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
