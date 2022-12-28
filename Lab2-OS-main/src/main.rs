#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buf;

use core::fmt::Write;
use core::panic::PanicInfo;
// імпортуємо структури Alignment, Color та Screen з модуля vga_buf
use crate::vga_buf::{Alignment, Color, Screen};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! { // точка входу для нашої ОС

    //створюємо екземпляр структури Screen, передаючи як параметри колір шрифту та тип вирівнювання
    let mut screen = Screen::new(Color::LIGHT_GREEN as u8, Alignment::Center);

    // 100 раз неявно викликаємо нашу функцію print через макрос write!
    for i in 0..100 {
        write!(screen, "Number {}\n", i);
    }

    loop {}
}
