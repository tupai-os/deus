pub mod vga;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod x86_ps2_kbd;
