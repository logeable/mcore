use crate::batch::run_next_app;
use crate::sbi::shutdown;

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
