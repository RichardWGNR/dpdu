use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock};

pub static PDU_STATE: LazyLock<Arc<PduState>> = LazyLock::new(|| Arc::default());

#[derive(Debug, Default)]
pub struct PduState {
    pub constructed: AtomicBool,
    pub api_tag: AtomicUsize,
}

impl PduState {
    pub fn set_constructed(&self, status: bool) -> &Self {
        self.constructed.store(status, Ordering::Relaxed);
        self
    }

    pub fn is_constructed(&self) -> bool {
        self.constructed.load(Ordering::Relaxed)
    }

    pub fn set_api_tag(&self, tag: usize) -> &Self {
        self.api_tag.store(tag, Ordering::Relaxed);
        self
    }

    pub fn destruct(&self) {
        self.set_constructed(false);
        self.set_api_tag(0);
    }
}