const MAX_APP_NUM: usize = 10;

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
            println!("app[{}]: {:#x}", i, self.app_start[i]);
        }
    }
}
