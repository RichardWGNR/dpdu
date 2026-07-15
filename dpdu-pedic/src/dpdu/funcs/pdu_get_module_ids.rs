use std::ffi::CString;
use dpdu_api_types::{ModuleData, ModuleItem, PduError, PduIt, PduStatus};
use crate::dpdu::state::PDU_STATE;
use crate::passthru::PassthruModule;
use crate::utils::{is_valid_ptr, prepare_vector};

#[unsafe(no_mangle)]
pub unsafe extern "system-unwind" fn PDUGetModuleIds(
    p_module_id_list: *mut *mut ModuleItem
) -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    } else if !is_valid_ptr(p_module_id_list) {
        return PduError::InvalidParameters;
    }

    let mut pdu_modules = vec![];

    for (id, passthru_module) in PassthruModule::load().iter().enumerate() {
        let name = passthru_module.name.replace("'", "");
        let vendor = passthru_module
            .vendor
            .clone()
            .unwrap_or_else(|| "unknown".to_string())
            .replace("'", "");
        let version = passthru_module.product_version.clone()
            .unwrap_or_else(|| "unknown".to_string())
            .replace("'", "");

        let name = CString::new(format!("VendorName='{vendor}' ModuleName='{name}' J2534StandardVersion='{version}'"))
            .expect("CString::new() failed");

        let info = CString::new("ConnectionType='unknown'")
            .expect("CString::new() failed");

        pdu_modules.push(ModuleData {
            module_type_id: 0,
            h_mod: id as _,
            vendor_module_name: name.into_raw() as _,
            vendor_additional_info: info.into_raw() as _,
            status: passthru_module.get_status(),
        });
    }

    prepare_vector(&mut pdu_modules);

    let (modules_ptr, len, _) = pdu_modules.into_raw_parts();
    let item = Box::new(ModuleItem {
        item_type: PduIt::ModuleId,
        num_entries: len as _,
        p_module_data: modules_ptr as _,
    });

    unsafe {
        *p_module_id_list = Box::into_raw(item);
    }

    PduError::StatusNoError
}