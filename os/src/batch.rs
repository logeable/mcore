use core::arch::asm;

const MAX_APP_NUM: usize = 10;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

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

    pub fn run_next(&mut self) {
        unsafe { self.load_app(self.current_app) };
        self.current_app = (self.current_app + 1) % self.num_app;
    }
}
