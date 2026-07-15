use dpdu_api_types::PduError;
use crate::dpdu::logical_links::LogicalLink;
use crate::dpdu::state::PDU_STATE;
use crate::passthru::PassthruModule;

#[unsafe(no_mangle)]
pub extern "system" fn PDUDestroyComLogicalLink(
    h_mod: u32,
    h_cll: u32
) -> PduError {
    if PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    }
    
    let Some(_) = PassthruModule::get(h_mod as _) else {
        return PduError::InvalidHandle;
    };
    let Some(_) = LogicalLink::get(h_cll as _) else {
        return PduError::InvalidHandle;
    };
    
    if LogicalLink::destroy(h_cll as _) {
        PduError::StatusNoError
    } else {
        PduError::FctFailed
    }
}