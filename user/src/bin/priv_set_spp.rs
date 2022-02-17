#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use riscv::register::sstatus;
use riscv::register::sstatus::SPP;

#[no_mangle]
fn main() -> i32 {
    println!("privileged instruction");
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    return 0;
}
