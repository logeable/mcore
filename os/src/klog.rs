use core::fmt::Arguments;
use log::{self, set_logger, set_max_level, Level, LevelFilter, SetLoggerError};

const ERROR_RED: u8 = 31;
const WARN_YELLOW: u8 = 93;
const INFO_BLUE: u8 = 34;
const DEBUG_GREEN: u8 = 32;
const TRACE_GRAY: u8 = 90;

struct KernelLogger;

static LOGGER: KernelLogger = KernelLogger;

macro_rules! with_color {
    ($args: ident, $color: ident) => {
        format_args!("\x1b[{}m{}\x1b[0m", $color, $args,);
    };
}

impl log::Log for KernelLogger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        print_with_color(
            format_args!("{} - {}", record.level(), record.args()),
            level_to_color(record.level()),
        );
    }
    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(&LOGGER).map(|()| set_max_level(level_filter_from_compile_env()))
}

fn print_with_color(fmt: Arguments, color: u8) {
    println!("{}", with_color!(fmt, color));
}

fn level_to_color(level: Level) -> u8 {
    match level {
        Level::Trace => TRACE_GRAY,
        Level::Debug => DEBUG_GREEN,
        Level::Info => INFO_BLUE,
        Level::Warn => WARN_YELLOW,
        Level::Error => ERROR_RED,
    }
}

fn level_filter_from_compile_env() -> LevelFilter {
    match option_env!("LOG") {
        level => str_to_level_filter(level.unwrap_or("off")),
    }
}

fn str_to_level_filter(level: &str) -> LevelFilter {
    match level {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Off,
    }
}
