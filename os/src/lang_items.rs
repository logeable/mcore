use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panic at: {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        )
    } else {
        println!("Panic: {}", info.message().unwrap());
    }
    shutdown();
}
