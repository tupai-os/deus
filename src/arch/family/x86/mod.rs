pub mod irq;
pub mod pic;
pub mod pit;
pub mod kbd;

pub fn init_hw() {
    irq::init();
    pic::init();
    pit::init();
}
