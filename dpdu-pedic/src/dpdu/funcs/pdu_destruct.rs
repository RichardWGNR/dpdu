use dpdu_api_types::PduError;
use crate::dpdu::state::PDU_STATE;

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUDestruct() -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    }

    PDU_STATE.destruct();

    PduError::StatusNoError
}