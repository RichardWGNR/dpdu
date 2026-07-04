use std::sync::{Arc, Weak};
use dpdu_api_types::PduStatus;
use crate::api::Api;
use crate::types::pdu_status::PduStatusData;
use crate::types::PduModuleHandle;

pub type VciList = Vec<Arc<PduVci>>;

#[derive(Debug, Clone)]
pub struct PduVci {
    pub(crate) api: Weak<Api>,

    pub(crate) h_mod: PduModuleHandle,

    pub(crate) module_name: Option<String>,

    pub(crate) additional_info: Option<String>,

    pub(crate) status: PduStatusData,
}

impl PduVci {
    pub fn get_handle(&self) -> PduModuleHandle {
        self.h_mod
    }

    pub fn get_name(&self) -> Option<&String> {
        self.module_name.as_ref()
    }

    pub fn get_additional_info(&self) -> Option<&String> {
        self.additional_info.as_ref()
    }

    pub fn is_available_for_connection(&self) -> bool {
        match self.status.status_code {
            PduStatus::ModstReady | PduStatus::ModstAvail => true,
            _ => false
        }
    }
}