#![allow(non_snake_case)]

use std::ffi::c_void;
use dpdu_api_types::{CopCtrlData, EventCallbackFn, EventItem, FlagData, ModuleItem, ParamItem, PduCopt, PduDataItem, PduError, PduErrorEvt, PduItem, PduObjt, PduStatus, RscConflictItem, RscData, RscIdItem, RscStatusItem, UniqueRespIdTableItem, VersionData};

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUConstruct() -> PduError {
    PduError::FctFailed
}

#[unsafe(no_mangle)]
pub extern "system-unwind" fn PDUDestruct() -> PduError {
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
pub extern "system" fn PDUDestroyItem(
    p_item: *mut PduItem
) -> PduError {
    PduError::FctFailed
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
pub extern "system" fn PDUGetModuleIds(
    p_module_id_list: *mut *mut ModuleItem
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