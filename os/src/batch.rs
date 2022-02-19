use crate::shutdown;
use crate::trap::TrapContext;
use core::arch::asm;
use core::cell::RefCell;
use core::cell::RefMut;

const MAX_APP_NUM: usize = 10;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;
const KERNEL_STACK_SIZE: usize = 0x2000;
const USER_STACK_SIZE: usize = 0x2000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}
impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + self.data.len()
    }
    pub fn push_context(&self, ctx: TrapContext) -> &TrapContext {
        let ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *ptr = ctx;
        }
        unsafe { ptr.as_mut().unwrap() }
    }
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}
impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + self.data.len()
    }
}

pub struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn new() -> Self {
        extern "C" {
            fn _num_app();
        }
        let num_app_ptr = _num_app as usize as *const usize;
        let num_app = unsafe { num_app_ptr.read_volatile() };
        let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
        let app_start_raw: &[usize] =
            unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
        app_start[..=num_app].copy_from_slice(app_start_raw);
        Self {
            num_app,
            current_app: 0,
            app_start,
        }
    }

    pub fn dump(&self) {
        println!("num_app: {}", self.num_app);
        println!("cur_app: {}", self.current_app);
        for i in 0..self.num_app {
            println!(
                "app_{}: [{:#x} - {:#x}] {}B",
                i,
                self.app_start[i],
                self.app_start[i + 1],
                self.app_start[i + 1] - self.app_start[i]
            );
        }
    }

    unsafe fn load_app(&self, id: usize) {
        if id >= self.num_app {
            panic!("id {} invalid", id);
        }
        println!("os loading app_{}", id);
        asm!("fence.i");

        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as usize as *mut u8, APP_SIZE_LIMIT)
            .fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[id] as *const u8,
            self.app_start[id + 1] - self.app_start[id],
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());

        app_dst.copy_from_slice(app_src);
    }
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

lazy_static! {
    static ref APP_MGR: MyRefCell<AppManager> = MyRefCell::new(AppManager::new());
}
struct MyRefCell<T> {
    inner: RefCell<T>,
}
unsafe impl<T> Sync for MyRefCell<T> {}
impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

pub fn init() {
    APP_MGR.exclusive_access().dump();
}
pub fn run_next_app() -> ! {
    let mut app_mgr = APP_MGR.exclusive_access();
    if app_mgr.current_app >= app_mgr.num_app {
        println!("all app loaded");
        shutdown();
    }
    unsafe { app_mgr.load_app(app_mgr.current_app) };
    app_mgr.current_app += 1;
    drop(app_mgr);
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        let ctx = KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        ));
        __restore(ctx as *const _ as usize);
    }
    panic!("unreachable code");
}
