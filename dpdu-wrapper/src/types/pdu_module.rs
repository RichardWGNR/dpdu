use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::types::{PduModuleHandle, PduObjectId};
use dpdu_api_types::PduStatus;

pub type PduModuleList = Vec<PduModule>;

pub type PduModulesResourcesIds = HashMap<PduModuleHandle, Vec<PduObjectId>>;

pub type PduConflictingModules = HashMap<PduModuleHandle, PduObjectId>;

#[derive(Debug, Clone)]
pub struct PduModule {
    pub h_mod: PduModuleHandle,

    pub module_type_id: u32,

    pub vendor_module_name: Option<String>,

    pub vendor_additional_info: Option<String>,

    pub status: PduStatus,
}