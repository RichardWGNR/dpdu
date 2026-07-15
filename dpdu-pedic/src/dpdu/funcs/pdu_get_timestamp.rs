use dpdu_api_types::PduError;
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::timestamp::PduTimestamp;
use crate::utils::is_valid_ptr;

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetTimestamp(
    _h_mod: u32,
    p_timestamp: *mut u32
) -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    } else if !is_valid_ptr(p_timestamp) {
        return PduError::InvalidParameters;
    }

    unsafe {
        *p_timestamp = PduTimestamp::now();
    }

    PduError::StatusNoError
}