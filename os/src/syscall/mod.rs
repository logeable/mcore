use crate::batch::run_next_app;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    match id {
        SYSCALL_WRITE => sys_write(arg0, arg1 as *const u8, arg2),
        SYSCALL_EXIT => sys_exit(arg0 as i32),
        _ => {
            panic!("not supported syscall id: {}", id);
        }
    }
}

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let s = core::str::from_utf8(slice).unwrap();
            print!("{}", s);
            len as isize
        }
        _ => {
            panic!("unsupported fd: {}", fd);
        }
    }
}

pub fn sys_exit(state: i32) -> ! {
    println!("app exit with {}", state);
    run_next_app();
}
