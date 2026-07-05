use dpdu_api_types::PduErrorEvt;
use crate::types::PduCopHandle;

#[derive(Debug, Clone)]
pub struct PduErrorData {
    pub error_event: PduErrorEvt,

    pub h_cop: Option<PduCopHandle>,

    pub timestamp: u32,

    pub extra_info_code: Option<u32>,
}