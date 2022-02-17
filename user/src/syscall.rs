#![allow(unused)]
use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

#[inline(always)]
fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret;
    unsafe {
        asm!(
        "ecall",
        inlateout("x10") arg0 =>ret,
        in("x11") arg1,
        in("x12") arg2,
        in("x17") id,
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, fd, buffer.as_ptr() as usize, buffer.len())
}
pub fn sys_exit(state: i32) -> ! {
    syscall(SYSCALL_EXIT, state as usize, 0, 0);
    panic!("unreachable code")
}
