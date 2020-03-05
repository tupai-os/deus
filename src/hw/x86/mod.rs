pub mod irq;
pub mod pic;
pub mod pit;

pub fn init() {
    irq::init();
    pic::init();
    pit::init();
}
