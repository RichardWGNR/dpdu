use crate::types::pdu_com_logical_link::{CllCreateFlags, CllCreateType, PduCllData};
use crate::types::pdu_com_param::PduComParam;
use crate::types::pdu_com_param_table::PduComParamTable;
use crate::types::pdu_com_primitive::{PduComPrimitiveParams, PduCopData};
use crate::types::pdu_error::{PduErrorData, PduLastErrorTarget};
use crate::types::pdu_event::{PduEvent, PduEventTarget};
use crate::types::pdu_io_ctl::{PduIoCtlCommand, PduIoCtlData, PduIoCtlTarget};
use crate::types::pdu_lock_resource::PduLockResourceMask;
use crate::types::pdu_module::{
    PduConflictingModules, PduModuleData, PduModuleList, PduModulesResourcesIds,
};
use crate::types::pdu_object::PduObjectIdSource;
use crate::types::pdu_resource::{
    BusSource, PduResource, PduResourceStatus, ProtocolSource, TargetPin,
};
use crate::types::pdu_status::{PduStatusData, PduStatusTarget};
use crate::types::pdu_version::PduVersionData;
use crate::types::{PduCllHandle, PduCopHandle, PduModuleHandle, PduObjectId, PduUniqueCllTag, PduUniqueCopTag};
use crate::utils::UnsafePtr;
use dpdu_api_types::{EventCallbackFn, PduCopt, PduObjt};
use dpdu_wrapper_support::declare_worker_rpc;
use std::ffi::c_void;
use crate::types::pdu_vci::VciList;

declare_worker_rpc! {
    // Virtual functions.
    VciList => get_vci_list() -> VciList,

    // Real D-PDU functions.
    PduCancelComPrimitive => pdu_cancel_com_primitive(h_mod: PduModuleHandle, h_cll: PduCllHandle, h_cop: PduCopHandle) -> (),
    PduConnect => pdu_connect(h_mod: PduModuleHandle, h_cll: PduCllHandle) -> (),
    PduConstruct => pdu_construct() -> (),
    PduCreateComLogicalLink => pdu_create_com_logical_link(h_mod: PduModuleHandle, create_type: CllCreateType, create_flags: CllCreateFlags, tag: Option<PduUniqueCllTag>) -> PduCllData,
    PduDestruct => pdu_destruct() -> (),
    PduDestroyComLogicalLink => pdu_destroy_com_logical_link(h_mod: PduModuleHandle, h_cll: PduCllHandle) -> (),
    PduDestroyItem => pdu_destroy_item(ptr: UnsafePtr<c_void>) -> (),
    PduDisconnect => pdu_disconnect(h_mod: PduModuleHandle, h_cll: PduCllHandle) -> (),
    PduGetComParam => pdu_get_com_param(h_mod: PduModuleHandle, h_cll: PduCllHandle, object_id: PduObjectIdSource) -> PduComParam,
    PduGetConflictingResources => pdu_get_conflicting_resources(resource_id: PduObjectId, modules: Vec<PduModuleData>) -> PduConflictingModules,
    PduGetEventItem => pdu_get_event_item(target: PduEventTarget) -> Option<PduEvent>,
    PduGetLastError => pdu_get_last_error(target: PduLastErrorTarget) -> PduErrorData,
    PduGetModuleIds => pdu_get_module_ids() -> PduModuleList,
    PduGetObjectId => pdu_get_object_id(object: PduObjt, short_name: Into<String>) -> PduObjectId,
    PduGetResourceIds => pdu_get_resource_ids(h_mod: Option<PduModuleHandle>, bus: BusSource, protocol: ProtocolSource, pins: Vec<TargetPin>) -> PduModulesResourcesIds,
    PduGetResourceStatus => pdu_get_resource_status(resources: Vec<PduResource>) -> PduResourceStatus,
    PduGetStatus => pdu_get_status(target: PduStatusTarget) -> PduStatusData,
    PduGetTimestamp => pdu_get_timestamp(h_mod: PduModuleHandle) -> u32,
    PduGetUniqueRespIdTable => pdu_get_unique_resp_id_table(h_mod: PduModuleHandle, h_cll: PduCllHandle) -> PduComParamTable,
    PduGetVersion => pdu_get_version(h_mod: PduModuleHandle) -> PduVersionData,
    PduIoCtl => pdu_io_ctl(target: PduIoCtlTarget, command: PduIoCtlCommand, data: Option<PduIoCtlData>) -> Option<PduIoCtlData>,
    PduLockResource => pdu_lock_resource(h_mod: PduModuleHandle, h_cll: PduCllHandle, mask: PduLockResourceMask) -> (),
    PduModuleConnect => pdu_module_connect(h_mod: PduModuleHandle) -> (),
    PduModuleDisconnect => pdu_module_disconnect(h_mod: Option<PduModuleHandle>) -> (),
    PduRegisterEventCallback => pdu_register_event_callback(target: PduEventTarget, callback: Option<EventCallbackFn>) -> (),
    PduSetComParam => pdu_set_com_param(h_mod: PduModuleHandle, h_cll: PduCllHandle, cp: PduComParam) -> (),
    PduSetUniqueRespIdTable => pdu_set_unique_resp_id_table(h_mod: PduModuleHandle, h_cll: PduCllHandle, table: PduComParamTable) -> (),
    PduStartComPrimitive => pdu_start_com_primitive(h_mod: PduModuleHandle, h_cll: PduCllHandle, cop_type: PduCopt, data: Vec<u8>, params: Option<PduComPrimitiveParams>, tag: Option<PduUniqueCopTag>) -> PduCopData,
    PduUnlockResource => pdu_unlock_resource(h_mod: PduModuleHandle, h_cll: PduCllHandle, mask: PduLockResourceMask) -> (),
}
