#[cfg(feature = "cpu_amd64")]
pub mod amd64;
#[cfg(feature = "cpu_amd64")]
pub use amd64 as active;
