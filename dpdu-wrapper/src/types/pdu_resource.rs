use crate::types::{PduModuleHandle, PduObjectId};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PduResource {
    pub h_mod: PduModuleHandle,
    pub resource_id: PduObjectId,
}

impl PduResource {
    pub fn new(h_mod: PduModuleHandle, resource_id: PduObjectId) -> Self {
        PduResource { h_mod, resource_id }
    }

    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.h_mod
    }

    pub fn get_resource_id(&self) -> PduObjectId {
        self.resource_id
    }
}

#[derive(Debug, Clone)]
pub struct ResourceStatus {
    pub raw_status: u32,

    /// - false = resource not in use
    /// - true = resource in use
    pub busy: bool,

    /// - false = resource available
    /// - true = resource not available
    pub available: bool,

    /// - false = Transmit Queue is not locked
    /// - true = Transmit Queue is locked by a CLL. No other CLL except the
    ///       one which holds the lock is allowed to transmit on the physical
    ///       resource.
    pub transmit_queue_lock: bool,

    /// - false = Physical ComParams are not locked
    /// - true = Physical ComParams are locked by a CLL. No other CLL
    ///          except the one which holds the lock is allowed to change the
    ///          physical ComParams for the resource.
    pub physical_com_param_lock: bool,
}
