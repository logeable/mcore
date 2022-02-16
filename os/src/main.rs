#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod log;
mod sbi;

use crate::sbi::shutdown;
use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    info!("stext: {:#x}", stext as usize);
    info!("etext: {:#x}", etext as usize);
    info!("srodata: {:#x}", srodata as usize);
    info!("erodata: {:#x}", erodata as usize);
    info!("sdata: {:#x}", sdata as usize);
    info!("edata: {:#x}", edata as usize);
    info!("boot_stack: {:#x}", boot_stack as usize);
    info!("boot_stack_top: {:#x}", boot_stack_top as usize);
    info!("sbss: {:#x}", sbss as usize);
    info!("ebss: {:#x}", ebss as usize);

    debug!("hello world");
    error!("now shutdown");
    shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
