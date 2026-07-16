use dpdu_api_types::{PduError, PduStatus};
use crate::dpdu::logical_links::LogicalLink;
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::timestamp::PduTimestamp;
use crate::dpdu::types::pdu_status::PduStatusTarget;
use crate::dpdu::types::{PduCllHandle, PduCopHandle, PduModuleHandle};
use crate::passthru::PassthruModule;
use crate::utils::is_valid_ptr;

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUGetStatus(
    h_mod: PduModuleHandle,
    h_cll: PduCllHandle,
    h_cop: PduCopHandle,
    p_status_code: *mut PduStatus,
    p_timestamp: *mut u32,
    p_extra_info: *mut u32
) -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    } else if !is_valid_ptr(p_status_code)
        || !is_valid_ptr(p_timestamp)
        || !is_valid_ptr(p_extra_info)
    {
        return PduError::InvalidParameters;
    }

    let target = match PduStatusTarget::from_api(h_mod, h_cll, h_cop) {
        Ok(v) => v,
        Err(err) => {
            return err;
        }
    };

    let matched = match target {
        PduStatusTarget::Module(h_mod) => {
            let Some(module) = PassthruModule::get(h_mod as _) else {
                return PduError::InvalidHandle;
            };

            unsafe { *p_status_code = module.get_status(); }

            true
        },
        PduStatusTarget::LogicalLink(_, h_cll) => {
            let Some(link) = LogicalLink::get(h_cll) else {
                return PduError::InvalidHandle;
            };

            unsafe { *p_status_code = link.get_status(); }

            true
        },
        _ => false
    };

    if matched {
        unsafe {
            *p_timestamp = PduTimestamp::now();
            *p_extra_info = 0;
        }
        PduError::StatusNoError
    } else {
        PduError::FctFailed
    }
}