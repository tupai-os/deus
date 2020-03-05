use alloc::vec::Vec;
use bootloader::bootinfo::{MemoryRegionType};
use crate::{log::*, boot_info};

const DEUS_CORE_DRIVER_MAGIC: [u8; 16] = *b"DEUS_CORE_DRIVER";

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DriverInfo {
    magic: [u8; 16],
    name: &'static str,
    init: fn(),
}

impl DriverInfo {
    pub const fn new(
        name: &'static str,
        init: fn(),
    ) -> Self {
        Self {
            magic: DEUS_CORE_DRIVER_MAGIC,
            name,
            init,
        }
    }
}

pub fn kernel_regions() -> impl Iterator<Item=&'static [u8]> {
    logln!("Boot info at {:?}", boot_info() as *const _);
    logln!("kernel_regions at {:?}", kernel_regions as usize);
    boot_info()
        .memory_map
        .iter()
        .filter_map(|region| if let MemoryRegionType::Kernel = region.region_type {
            unsafe { Some(core::slice::from_raw_parts(
                (region.range.start_frame_number as usize * 4096) as *const _,
                (region.range.end_frame_number - region.range.start_frame_number) as usize * 4096,
            )) }
        } else {
            None
        })
}

pub fn init_drivers() {
    /*
    let drivers = kernel_regions()
        .map(|bytes| {
            logln!("At {:?}", bytes.as_ptr());
            bytes.windows(DEUS_CORE_DRIVER_MAGIC.len())
        })
        .flatten()
        .filter(|bytes| bytes == &DEUS_CORE_DRIVER_MAGIC)
        .map(|bytes| unsafe { *(bytes.as_ptr() as *const _) })
        .collect::<Vec<DriverInfo>>();
    */

    let drivers = vec![
        crate::driver::x86_ps2_kbd::DRIVER_INFO,
    ];

    for driver in drivers {
        logln!("[ OK ] Initiating driver {}...", driver.name);
        (driver.init)();
    }
}
