mod gdt;
mod idt;
mod isr;
mod fault;
mod port;

mod hw;

// Kernel
use crate::bootinfo::BootInfo;

pub mod cpu {
    // Library
    use spin::Once;

    // Kernel
    use crate::llapi::mach::traits;

    // TODO: Don't hard-code this
    static VENDOR: &str = "Intel";

    // Cpu

    pub struct Cpu {
        info: traits::CpuInfo,
        cores: [Core; 4],
    }

    impl Cpu {
        fn singleton() -> Self {
            Self {
                info: traits::CpuInfo { vendor: VENDOR },
                cores: [
                    Core::singleton(),
                    Core::singleton(),
                    Core::singleton(),
                    Core::singleton(),
                ],
            }
        }
    }

    impl traits::Cpu for Cpu {
        type Core = Core;

        fn info(&self) -> &traits::CpuInfo { &self.info }
        fn cores(&self) -> &[Self::Core] { &self.cores }
        fn primary_core(&self) -> &Self::Core { &self.cores[0] }
        fn this_core(&self) -> &Self::Core { &self.cores[0] }
    }

    // Core

    pub struct Core {
        info: traits::CoreInfo,
    }

    impl Core {
        const fn singleton() -> Self {
            Self {
                info: traits::CoreInfo,
            }
        }
    }

    impl traits::Core for Core {
        fn info(&self) -> &traits::CoreInfo { &self.info }
        unsafe fn enable_irqs(&self) { asm!("sti"); }
        unsafe fn disable_irqs(&self) { asm!("sti"); }
        fn await_irq(&self) { unsafe { asm!("hlt"); } }
    }

    static SINGLETON: Once<Cpu> = Once::new();

    pub fn singleton() -> &'static Cpu {
        SINGLETON.call_once(|| Cpu::singleton())
    }
}

// _start

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    idt::init();
    fault::init();
    hw::irq::init();
    hw::pic::init();

    crate::kernel_entry(BootInfo)
}
