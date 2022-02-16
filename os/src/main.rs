#![no_std]
#![no_main]

mod lang_items;
mod sbi;

use core::arch::global_asm;

use sbi::console_putchar;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    let str = "hello world\n";
    for &ele in str.as_bytes() {
        console_putchar(ele as usize);
    }
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
