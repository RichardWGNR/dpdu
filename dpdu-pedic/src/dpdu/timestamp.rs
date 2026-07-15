use std::sync::LazyLock;
use std::time::Instant;
use parking_lot::RwLock;

static START: LazyLock<RwLock<Instant>> = LazyLock::new(|| RwLock::new(Instant::now()));

#[derive(Debug)]
pub struct PduTimestamp {
    start: Instant,
}

impl PduTimestamp {
    /// Reset time base (PDU_IOCTL_RESET / device boot)
    pub fn reset() {
        let mut start = START.write();
        *start = Instant::now();
    }

    pub fn now() -> u32 {
        let start = START.read();
        start.elapsed().as_micros() as u32
    }
}