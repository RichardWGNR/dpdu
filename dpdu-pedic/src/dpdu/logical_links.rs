use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use dpdu_api_types::PduStatus;
use parking_lot::{Mutex, RwLock};
use rand::random;
use crate::dpdu::types::{PduCllHandle, PduModuleHandle, PduObjectId, PduUniqueCllTag};

static LOGICAL_LINKS: LazyLock<RwLock<HashMap<PduCllHandle, Arc<LogicalLink>>>> = LazyLock::new(|| RwLock::default());

#[derive(Debug)]
pub struct LogicalLink {
    pub h_mod: PduModuleHandle,
    pub h_cll: PduCllHandle,
    pub tag: Option<PduUniqueCllTag>,
    pub protocol_id: PduObjectId,
    pub bus_id: PduObjectId,
    pub status: RwLock<PduStatus>,
}

impl Default for LogicalLink {
    fn default() -> Self {
        Self {
            h_mod: Default::default(),
            h_cll: Default::default(),
            tag: Default::default(),
            protocol_id: Default::default(),
            bus_id: Default::default(),
            status: RwLock::new(PduStatus::CllstOffline)
        }
    }
}

impl LogicalLink {
    pub fn get_tag(&self) -> usize {
        self.tag.as_ref().unwrap().get()
    }

    pub fn get(id: PduCllHandle) -> Option<Arc<LogicalLink>> {
        let logical_links = LOGICAL_LINKS.read();
        logical_links.get(&id).cloned()
    }

    pub fn destroy(id: PduCllHandle) -> bool {
        // TODO : close all resources and send event
        let mut logical_links = LOGICAL_LINKS.write();
        logical_links.remove(&id).is_some()
    }

    pub fn register(mut link: LogicalLink) -> PduCllHandle {
        let h_cll = random();
        link.h_cll = h_cll;

        let mut logical_links = LOGICAL_LINKS.write();
        logical_links.insert(h_cll, Arc::new(link));

        h_cll
    }

    pub fn set_status(&self, status: PduStatus) {
        *self.status.write() = status;
    }

    pub fn get_status(&self) -> PduStatus {
        self.status.read().clone()
    }
}