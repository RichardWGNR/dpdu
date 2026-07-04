use dpdu_api_types::PduStatus;
use crate::types::{PduCllHandle, PduCopHandle, PduModuleHandle};

#[derive(Debug, Clone)]
pub struct PduStatusData {
    pub h_mod: PduModuleHandle,
    pub h_cll: Option<PduCllHandle>,
    pub h_cop: Option<PduCopHandle>,
    pub status_code: PduStatus,
    pub timestamp: u32,
    pub extra_info: u32
}