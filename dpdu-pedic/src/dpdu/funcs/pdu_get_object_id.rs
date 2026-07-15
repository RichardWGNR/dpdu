use std::ffi::CStr;
use dpdu_api_types::{PduError, PduObjt, PDU_ID_UNDEF};
use crate::dpdu::objects::{BUSES, PINS, PROTOCOLS};
use crate::dpdu::state::PDU_STATE;
use crate::utils::is_valid_ptr;

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUGetObjectId(
    pdu_object_type: PduObjt,
    p_short_name: *const u8,
    p_pdu_object_id: *mut u32
) -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    } else if !is_valid_ptr(p_short_name) || !is_valid_ptr(p_pdu_object_id) {
        return PduError::InvalidParameters;
    }

    let short_name = unsafe {
        CStr::from_ptr(p_short_name as _)
            .to_string_lossy()
            .to_string()
    };

    let id = match pdu_object_type {
        PduObjt::BusType => BUSES.get(short_name.as_str()).map(u32::to_owned),
        PduObjt::Protocol => PROTOCOLS.get(short_name.as_str()).map(u32::to_owned),
        PduObjt::PinType => PINS.get(short_name.as_str()).map(u32::to_owned),
        _ => None
    };

    unsafe { *p_pdu_object_id = id.unwrap_or(PDU_ID_UNDEF) };

    PduError::StatusNoError
}