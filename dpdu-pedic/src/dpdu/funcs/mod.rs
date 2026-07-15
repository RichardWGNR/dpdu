#![allow(non_snake_case)]

use std::ffi::{c_void, CString};
use dpdu_api_types::{CopCtrlData, EventCallbackFn, EventItem, FlagData, ModuleData, ModuleItem, ParamItem, PduCopt, PduDataItem, PduError, PduErrorEvt, PduIt, PduItem, PduObjt, PduStatus, RscConflictItem, RscData, RscIdItem, RscStatusItem, UniqueRespIdTableItem, VersionData};

mod pdu_get_module_ids;
mod pdu_destroy_item;
mod pdu_construct;
mod pdu_destruct;
mod pdu_get_status;
mod pdu_module_connect;
mod pdu_module_disconnect;
mod pdu_get_object_id;
mod pdu_get_timestamp;
mod pdu_create_com_logical_link;
mod pdu_destroy_com_logical_link;
mod pdu_register_event_callback;

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