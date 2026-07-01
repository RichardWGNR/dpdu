use dpdu_api_types::PDU_HANDLE_UNDEF;
use crate::types::{PduCllHandle, PduModuleHandle};

#[derive(Debug, Clone)]
#[derive(strum::AsRefStr)]
pub enum PduEventCallbackTarget {
    System,
    Module(PduModuleHandle),
    ComLogicalLink(PduModuleHandle, PduCllHandle)
}