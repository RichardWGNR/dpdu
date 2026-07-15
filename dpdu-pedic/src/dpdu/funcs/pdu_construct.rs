use std::ffi::c_void;
use dpdu_api_types::PduError;
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::timestamp::PduTimestamp;

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUConstruct(
    option_str: *mut u8,
    p_api_tag: *mut c_void
) -> PduError {
    if PDU_STATE.is_constructed() {
        return PduError::FctFailed;
    }

    PDU_STATE
        .set_api_tag(p_api_tag as *const usize as usize)
        .set_constructed(true);

    PduTimestamp::reset();
    
    PduError::StatusNoError
}