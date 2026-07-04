use dpdu_api_types::PduStatus;
use crate::types::PduModuleHandle;

pub type PduModuleList = Vec<PduModule>;

#[derive(Debug, Clone)]
pub struct PduModule {
    pub h_mod: PduModuleHandle,
    
    pub module_type_id: u32,
    
    pub vendor_module_name: Option<String>,
    
    pub vendor_additional_info: Option<String>,
    
    pub status: PduStatus
}