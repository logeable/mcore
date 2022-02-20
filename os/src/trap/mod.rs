mod context;

use crate::batch::run_next_app;
use crate::syscall;
pub use context::TrapContext;
use core::arch::global_asm;
use riscv::register::scause::Exception;
use riscv::register::scause::Trap;
use riscv::register::{scause, stval, stvec};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.sepc += 4;
            ctx.x[10] = syscall::syscall(ctx.x[17], ctx.x[10], ctx.x[11], ctx.x[12]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] page fault, shutdown");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] illegal instruction, shutdown");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}",
                scause.cause(),
                stval
            );
        }
    }
    ctx
}
