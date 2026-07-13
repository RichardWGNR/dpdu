use crate::types::{PduModuleHandle, PduObjectId};
use dpdu_api_types::PduStatus;
use std::collections::HashMap;

pub type PduModuleList = Vec<PduModuleData>;

pub type PduModulesResourcesIds = HashMap<PduModuleHandle, Vec<PduObjectId>>;

pub type PduConflictingModules = HashMap<PduModuleHandle, PduObjectId>;

#[derive(Debug, Clone)]
pub struct PduModuleData {
    pub(crate) h_mod: PduModuleHandle,

    pub(crate) module_type_id: u32,

    pub(crate) vendor_module_name: Option<String>,

    pub(crate) vendor_additional_info: Option<String>,

    pub(crate) status: PduStatus,
}

impl PduModuleData {
    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.h_mod
    }
    
    pub fn get_module_type_id(&self) -> u32 {
        self.module_type_id
    }
    
    pub fn get_vendor_module_name(&self) -> Option<&String> {
        self.vendor_module_name.as_ref()
    }
    
    pub fn get_vendor_additional_info(&self) -> Option<&String> {
        self.vendor_additional_info.as_ref()
    }
    
    pub fn get_status(&self) -> &PduStatus {
        &self.status
    }
}
