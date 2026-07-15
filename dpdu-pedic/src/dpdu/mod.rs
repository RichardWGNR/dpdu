#![allow(non_snake_case)]

mod state;

use std::ffi::{c_void, CString};
use std::ptr;
use dpdu_api_types::{CopCtrlData, EventCallbackFn, EventItem, FlagData, ModuleData, ModuleItem, ParamItem, PduCopt, PduDataItem, PduError, PduErrorEvt, PduIt, PduItem, PduObjt, PduStatus, RscConflictItem, RscData, RscIdItem, RscStatusItem, UniqueRespIdTableItem, VersionData};
use crate::dpdu::state::PDU_STATE;
use crate::utils::is_valid_ptr;

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

    PduError::StatusNoError
}

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUDestruct() -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    }

    PDU_STATE.destruct();

    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUIoCtl(
    h_mod: u32,
    h_cll: u32,
    cmd_id: u32,
    p_input_data: *const PduDataItem,
    p_output_data: *mut *mut PduDataItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUGetVersion(
    h_mod: u32,
    p_version_data: *mut VersionData
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUGetStatus(
    h_mod: u32,
    h_cll: u32,
    h_cop: u32,
    p_status_code: *mut PduStatus,
    p_timestamp: *mut u32,
    p_extra_info: *mut u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUGetLastError(
    h_mod: u32,
    h_cll: u32,
    p_error_code: *mut PduErrorEvt,
    ph_cop: *mut u32,
    p_timestamp: *mut u32,
    p_extra_error_info: *mut u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetResourceStatus(
    p_resource_status: *mut RscStatusItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUCreateComLogicalLink(
    h_mod: u32,
    p_rsc_data: *const RscData,
    resource_id: u32,
    p_cll_tag: *const c_void,
    ph_cll: *mut u32,
    p_cll_create_flag: *mut FlagData
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUDestroyComLogicalLink(
    h_mod: u32,
    h_cll: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUConnect(
    h_mod: u32,
    h_cll: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUDisconnect(
    h_mod: u32,
    h_cll: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDULockResource(
    h_mod: u32,
    h_cll: u32,
    lock_mask: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUUnlockResource(
    h_mod: u32,
    h_cll: u32,
    lock_mask: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetComParam(
    h_mod: u32,
    h_cll: u32,
    param_id: u32,
    p_param_item: *mut *mut ParamItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUSetComParam(
    h_mod: u32,
    h_cll: u32,
    p_param_items: *mut ParamItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUStartComPrimitive(
    h_mod: u32,
    h_cll: u32,
    cop_type: PduCopt,
    cop_data_size: u32,
    p_cop_data: *mut u8,
    p_cop_ctrl_data: *mut CopCtrlData,
    p_cop_tag: *mut c_void,
    ph_cop: *mut u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUCancelComPrimitive(
    h_mod: u32,
    h_cll: u32,
    h_cop: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetEventItem(
    h_mod: u32,
    h_cll: u32,
    p_event_item: *mut *mut EventItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn PDUDestroyItem(
    p_item: *mut PduItem
) -> PduError {
    if !is_valid_ptr(p_item) {
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

#[unsafe(no_mangle)]
pub extern "system" fn PDURegisterEventCallback(
    h_mod: u32,
    h_cll: u32,
    callback_fn: EventCallbackFn
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetObjectId(
    pdu_object_type: PduObjt,
    p_short_name: *const u8,
    p_pdu_object_id: *mut u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn PDUGetModuleIds(
    p_module_id_list: *mut *mut ModuleItem
) -> PduError {
    if !is_valid_ptr(p_module_id_list) {
        return PduError::InvalidParameters;
    }

    let name = CString::new("VendorName='ALLScanner' ModuleName='VXDIAG' J2534 Standard Version='4.04'").unwrap();
    let info = CString::new("ConnectionType='unknown'").unwrap();

    let (modules_ptr, len, _) = vec![ModuleData {
        module_type_id: 0,
        h_mod: 0,
        vendor_module_name: name.into_raw() as _,
        vendor_additional_info: info.into_raw() as _,
        status: PduStatus::ModstAvail,
    }].into_raw_parts();

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

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetResourceIds(
    h_mod: u32,
    p_resource_id_data: *mut RscData,
    p_resource_id_list: *mut *mut RscIdItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetConflictingResources(
    resource_id: u32,
    p_input_module_list: *mut ModuleItem,
    p_output_conflict_list: *mut *mut RscConflictItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetUniqueRespIdTable(
    h_mod: u32,
    h_cll: u32,
    p_unique_resp_id_table: *mut *mut UniqueRespIdTableItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUSetUniqueRespIdTable(
    h_mod: u32,
    h_cll: u32,
    p_unique_resp_id_table: *mut UniqueRespIdTableItem
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUModuleConnect(
    h_mod: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUModuleDisconnect(
    h_mod: u32
) -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system" fn PDUGetTimestamp(
    h_mod: u32,
    p_timestamp: *mut u32
) -> PduError {
    PduError::FctFailed
}