use std::ffi::CString;
use dpdu_api_types::{ModuleItem, PduError, PduIt, PduItem};
use crate::dpdu::state::PDU_STATE;
use crate::utils::is_valid_ptr;

#[unsafe(no_mangle)]
pub unsafe extern "system-unwind" fn PDUDestroyItem(
    p_item: *mut PduItem
) -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    } else if !is_valid_ptr(p_item) {
        return PduError::InvalidParameters;
    }
    
    let item = unsafe { &*p_item };

    match item.item_type {
        PduIt::ModuleId => {
            let item = unsafe { Box::from_raw(p_item as *mut ModuleItem) };
            if item.num_entries > 0 && is_valid_ptr(item.p_module_data) {
                let modules = unsafe { Vec::from_raw_parts(
                    item.p_module_data,
                    item.num_entries as _,
                    item.num_entries as _
                ) };

                for module in modules.iter() {
                    if is_valid_ptr(module.vendor_module_name) {
                        drop(unsafe { CString::from_raw(module.vendor_module_name as _) });
                    }
                    if is_valid_ptr(module.vendor_additional_info) {
                        drop(unsafe { CString::from_raw(module.vendor_additional_info as _) });
                    }
                }

                drop(modules);
            }

            drop(item);
        },
        _ => {
            return PduError::InvalidParameters;
        }
    }

    PduError::StatusNoError
}