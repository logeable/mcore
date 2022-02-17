#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod klog;
mod lang_items;
mod sbi;

use crate::sbi::shutdown;
use core::arch::global_asm;
use log::{debug, error, info};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    klog::init().unwrap();

    print_boot_info();

    print!("hello world\n");
    debug!("hello world");
    error!("now shutdown");
    shutdown();
}

fn print_boot_info() {
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
        fn skernel();
        fn ekernel();
        fn _start();
    }
    info!(".text: [{:#x} {:#x}]", stext as usize, etext as usize);
    info!(".rodata: [{:#x} {:#x}]", srodata as usize, erodata as usize);
    info!(".data: [{:#x} {:#x}]", sdata as usize, edata as usize);
    info!(".bss: [{:#x} {:#x}]", sbss as usize, ebss as usize);
    info!(
        "stack: [{:#x} {:#x}]",
        boot_stack as usize, boot_stack_top as usize
    );
    info!(
        "kernel: [{:#x} {:#x}] start = {:#x}",
        skernel as usize, ekernel as usize, _start as usize
    );
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
