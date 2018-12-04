// Library
use core::fmt::{self, Write};

// Kernel
use crate::vdev::tty;

#[allow(dead_code)]
pub fn log_args(args: fmt::Arguments) {
    tty::default_do_for_mut(|tty| tty.write_fmt(args))
        .expect("Could access default TTY")
        .expect("Could not log to default TTY");
}

#[allow(dead_code)]
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::log::log_args(format_args!($($arg)*)));
}

#[allow(dead_code)]
#[macro_export]
macro_rules! logln {
    () => (log!("\n"));
    ($fmt:expr) => ($crate::log!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::log!(concat!($fmt, "\n"), $($arg)*));
}
