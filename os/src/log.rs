const ERROR_RED: u8 = 31;
const WARN_YELLOW: u8 = 93;
const INFO_BLUE: u8 = 34;
const DEBUG_GREEN: u8 = 32;
const TRACE_GRAY: u8 = 90;

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[32m", $fmt, "\x1b[0m", "\n") $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m", $fmt, "\x1b[0m", "\n") $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m", $fmt, "\x1b[0m", "\n") $(, $($arg)+)?));
    };
}
