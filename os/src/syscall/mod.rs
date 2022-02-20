mod syscall;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    match id {
        SYSCALL_WRITE => syscall::sys_write(arg0, arg1 as *const u8, arg2),
        SYSCALL_EXIT => syscall::sys_exit(arg0 as i32),
        _ => {
            panic!("not supported syscall id: {}", id);
        }
    }
}
