use crate::types::pdu_com_logical_link::{CllCreateFlags, CllCreateType, PduCllData};
use crate::types::pdu_com_param::{
    ByteFieldComParam, CpVariant, LongFieldComParam, PduComParam, StructComParam,
    StructFieldComParam,
};
use crate::types::pdu_com_param_table::PduComParamTable;
use crate::types::pdu_com_primitive::{PduComPrimitiveParams, PduCopData};
use crate::types::pdu_error::{PduErrorData, PduLastErrorTarget};
use crate::types::pdu_event::{
    PduErrorEvent, PduEvent, PduEventData, PduEventTarget, PduInfoEvent, PduResultEvent,
    PduStatusEvent,
};
use crate::types::pdu_io_ctl::{IoCtlByteArray, PduIoCtlCommand, PduIoCtlData, PduIoCtlTarget};
use crate::types::pdu_lock_resource::PduLockResourceMask;
use crate::types::pdu_module::{
    PduConflictingModules, PduModuleData, PduModuleList, PduModulesResourcesIds,
};
use crate::types::pdu_object::PduObjectIdSource;
use crate::types::pdu_resource::{
    BusSource, PduResource, PduResourceStatus, PinSource, ProtocolSource, ResourceStatus, TargetPin,
};
use crate::types::pdu_status::{PduStatusData, PduStatusTarget};
use crate::types::pdu_version::PduVersionData;
use crate::types::{PduCllHandle, PduCopHandle, PduLibraryPath, PduModuleHandle, PduObjectId, PduOptions, PduUniqueApiTag, PduUniqueCllTag, PduUniqueCopTag};
use crate::utils::{c_str, random_non_zero_usize};
use crate::utils::module_description::{PduModuleDescription, PduModuleDescriptionError};
use crate::utils::root_file::Mvci;
use dpdu_api_types::{
    CopCtrlData, EcuUniqueRespData, ErrorData, EventCallbackFn, EventItem, ExpRespData, FlagData,
    InfoData, IoByteArrayData, IoEventQueuePropertyData, IoFilterData, IoProgVoltageData,
    ModuleData, ModuleItem, PDU_HANDLE_UNDEF, PDU_ID_UNDEF, ParamByteFieldData, ParamItem,
    ParamLongFieldData, ParamStructAccessTiming, ParamStructFieldData, ParamStructSessionTiming,
    PduCancelComPrimitiveFn, PduConnectFn, PduConstructFn, PduCopt, PduCpst,
    PduCreateComLogicalLinkFn, PduDestroyComLogicalLinkFn, PduDestroyItemFn, PduDestructFn,
    PduDisconnectFn, PduError, PduErrorEvt, PduGetComParamFn, PduGetConflictingResourcesFn,
    PduGetEventItemFn, PduGetLastErrorFn, PduGetModuleIdsFn, PduGetObjectIdFn, PduGetResourceIdsFn,
    PduGetResourceStatusFn, PduGetStatusFn, PduGetTimestampFn, PduGetUniqueRespIdTableFn,
    PduGetVersionFn, PduIoctlFn, PduIt, PduItem, PduLockResourceFn, PduModuleConnectFn,
    PduModuleDisconnectFn, PduObjt, PduPc, PduPt, PduRegisterCallbackFn, PduSetComParamFn,
    PduSetUniqueRespIdTableFn, PduStartComPrimitiveFn, PduStatus, PduUnlockResourceFn, PinData,
    ResultData, RscData, RscStatusData, RscStatusItem, UniqueRespIdTableItem, VersionData,
};
use rand::random;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::ffi::{CString, c_void};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::{Arc, Weak};
use std::{ptr, slice};
use std::num::NonZeroUsize;
use std::thread::spawn;
use tracing::{debug, error, trace, warn};
use crate::handle_manager::PduHandleManager;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("ffi error: {0}")]
    FfiError(#[from] libloading::Error),

    #[error("pdu error: {0}")]
    PduError(#[from] PduError),

    #[error("module description error: {0}")]
    MdfError(#[from] PduModuleDescriptionError),
}

#[derive(Debug)]
pub struct PduApi {
    pub(crate) me: Weak<PduApi>,

    pdu_options: PduOptions,

    pub(crate) unique_tag: PduUniqueApiTag,

    library: libloading::Library,

    library_file: Option<PduLibraryPath>,

    pub(crate) module_description: Option<PduModuleDescription>,

    mvci: Option<Mvci>,
}

impl PduApi {
    pub fn new(
        options: PduOptions,
        library: libloading::Library,
        library_file: Option<PduLibraryPath>,
        module_description: Option<PduModuleDescription>,
        mvci: Option<Mvci>,
    ) -> Arc<Self> {
        let tag = random_non_zero_usize();
        let result = Arc::new_cyclic(|me| Self {
            me: me.clone(),
            pdu_options: options,
            unique_tag: tag,
            library,
            library_file,
            module_description,
            mvci,
        });

        PduHandleManager::register_api(&result);

        result
    }

    pub fn from_mvci(mvci: &Mvci, options: PduOptions) -> ApiResult<Arc<Self>> {
        let library = unsafe { libloading::Library::new(&mvci.library_file)? };
        let mdf = mvci
            .module_description_file
            .as_ref()
            .map(|v| PduModuleDescription::parse_from_xml_file(v))
            .transpose()?;

        Ok(PduApi::new(
            options,
            library,
            Some(mvci.library_file.clone()),
            mdf,
            Some(mvci.clone()),
        ))
    }

    pub fn from_library_path(
        library_file: impl Into<PduLibraryPath>,
        options: PduOptions,
        module_description: Option<PduModuleDescription>,
    ) -> ApiResult<Arc<Self>> {
        let library_file = library_file.into();
        let library = unsafe { libloading::Library::new(&library_file)? };

        Ok(PduApi::new(
            options,
            library,
            Some(library_file),
            module_description,
            None,
        ))
    }

    pub fn from_library(
        library: libloading::Library,
        options: PduOptions,
        module_description: Option<PduModuleDescription>,
    ) -> ApiResult<Arc<Self>> {
        Ok(PduApi::new(
            options,
            library,
            None,
            module_description,
            None,
        ))
    }

    pub fn get_unique_tag(&self) -> PduUniqueApiTag {
        self.unique_tag
    }

    fn log_api_call(&self, func: &str) {
        debug!(func, "D-PDU API Call");
    }

    fn log_failed_api_call(&self, func: &str, result: PduError) {
        error!(
            func,
            result_str = %result,
            result_int = format!("{:#x}", result as usize),
            "D-PDU API Call failed"
        );
    }

    fn get_pdu_function<F>(&self, name: &[u8]) -> ApiResult<libloading::Symbol<'_, F>> {
        // SAFETY:
        // The caller must ensure that the requested symbol exists and that its
        // signature matches the D-PDU API specification.
        let result = unsafe { self.library.get(name) };

        match result {
            Ok(v) => Ok(v),
            Err(err) => {
                let function = String::from_utf8_lossy(name).into_owned();
                error!(
                    function,
                    "Unable to take a pointer to the D-PDU API function: {err}"
                );
                Err(err)?
            }
        }
    }

    pub fn pdu_construct(&self) -> ApiResult<()> {
        const FUNC: &'static str = "PDUConstruct";
        self.log_api_call(FUNC);

        let options_str = {
            // 9.4.2.4 Parameters
            // OptionStr String containing a list of attributes and their values. An attribute and its corresponding value
            // are to be separated by an >=< sign. The value needs to be put inside two >'< signs. Between
            // pairs of attribute and value shall be at least one space character. Attributes and values are
            // specific to a D-PDU API implementation.
            // When no option is to be set, the OptionStr can either be an empty string or NULL.
            //
            // 9.4.2.5 Example
            // OptionStr = "UseCaching='TRUE' InterfaceCheck='FALSE'"
            self.pdu_options
                .iter()
                .map(|(k, v)| format!("{k}='{v}'"))
                .collect::<Vec<String>>()
                .join(" ")
        };

        trace!(func = FUNC, options_str, "D-PDU API Call Args");

        let options_str = CString::new(options_str).expect("CString::new() failed");
        let construct_fn = self.get_pdu_function::<PduConstructFn>(FUNC.as_bytes())?;
        let result = construct_fn(
            options_str.as_ptr() as _,
            self.unique_tag.get() as *const PduUniqueApiTag as _,
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_destruct(&self) -> ApiResult<()> {
        const FUNC: &'static str = "PDUDestruct";
        self.log_api_call(FUNC);

        let destruct_fn = self.get_pdu_function::<PduDestructFn>(FUNC.as_bytes())?;

        match destruct_fn() {
            PduError::StatusNoError | PduError::PduApiNotConstructed => Ok(()),
            v => {
                self.log_failed_api_call(FUNC, v);
                Err(v)?
            }
        }
    }

    pub fn pdu_destroy_item(&self, item_ptr: *mut PduItem) -> ApiResult<()> {
        const FUNC: &'static str = "PDUDestroyItem";
        self.log_api_call(FUNC);

        trace!(
            func = FUNC,
            p_item = format!("0x{:x}", item_ptr as usize),
            "D-PDU API Call Args"
        );

        if item_ptr.is_null() {
            return Ok(());
        }

        let destroy_item_fn = self.get_pdu_function::<PduDestroyItemFn>(FUNC.as_bytes())?;
        let result = destroy_item_fn(item_ptr);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_get_event_item(&self, target: &PduEventTarget) -> ApiResult<Option<PduEvent>> {
        const FUNC: &'static str = "PDUGetEventItem";
        self.log_api_call(FUNC);

        let h_mod = target.get_module_handle().unwrap_or(PDU_HANDLE_UNDEF);
        let h_cll = target.get_cll_handle().unwrap_or(PDU_HANDLE_UNDEF);

        trace!(func = FUNC, h_mod, h_cll, "D-PDU API Call Args");

        let mut item_ptr: *mut EventItem = ptr::null_mut();

        let get_event_item_fn = self.get_pdu_function::<PduGetEventItemFn>(FUNC.as_bytes())?;
        let result = get_event_item_fn(h_mod, h_cll, &mut item_ptr);

        match result {
            PduError::StatusNoError | PduError::EventQueueEmpty => {}
            v => {
                self.log_failed_api_call(FUNC, result);
                return Err(v)?;
            }
        }

        trace!(
            func = FUNC,
            item_ptr = format!("0x{:x}", item_ptr as usize),
            item_type = ?NonNull::new(item_ptr).map(|wptr| unsafe { (&*wptr.as_ptr()).item_type }),
            "D-PDU API Call Return"
        );

        if item_ptr.is_null() {
            error!(
                func = FUNC,
                "Item pointer is null. Emulation of PduError::FctFailed..."
            );
            return Err(PduError::FctFailed)?;
        }

        let item = unsafe { &*item_ptr };

        if item.p_data.is_null() {
            error!(
                func = FUNC,
                "Item data pointer is null. Emulation of PduError::FctFailed..."
            );
            return Err(PduError::FctFailed)?;
        }

        let data: PduEventData = match item.item_type {
            PduIt::Status => PduStatusEvent(unsafe { *(item.p_data as *const PduStatus) }).into(),
            PduIt::Result => {
                let data = unsafe { &*(item.p_data as *const ResultData) };

                let mut extra_header = OnceCell::new();
                let mut extra_footer = OnceCell::new();

                if !data.p_extra_info.is_null() {
                    let extra_info = unsafe { &*data.p_extra_info };
                    if !extra_info.p_header_bytes.is_null() {
                        let ptr = extra_info.p_header_bytes;
                        let len = extra_info.num_header_bytes;
                        if !ptr.is_null() && len > 0 {
                            extra_header
                                .set(unsafe { slice::from_raw_parts(ptr, len as _) }.to_vec())
                                .unwrap();
                        }
                    }
                    if !extra_info.p_footer_bytes.is_null() {
                        let ptr = extra_info.p_footer_bytes;
                        let len = extra_info.num_footer_bytes;
                        if !ptr.is_null() && len > 0 {
                            extra_footer
                                .set(unsafe { slice::from_raw_parts(ptr, len as _) }.to_vec())
                                .unwrap();
                        }
                    }
                }

                PduResultEvent {
                    rx_flags: unsafe {
                        let ptr = data.rx_flag.p_flag_data;
                        let len = data.rx_flag.num_flag_bytes as usize;
                        let slice = if ptr.is_null() || len == 0 {
                            &[]
                        } else {
                            slice::from_raw_parts(ptr, len)
                        };
                        slice.to_vec().into()
                    },
                    unique_resp_identifier: data.unique_resp_identifier,
                    acceptance_id: data.acceptance_id,
                    timestamp_flags: unsafe {
                        let ptr = data.timestamp_flags.p_flag_data;
                        let len = data.timestamp_flags.num_flag_bytes as usize;
                        let slice = if ptr.is_null() || len == 0 {
                            &[]
                        } else {
                            slice::from_raw_parts(ptr, len)
                        };
                        slice.to_vec().into()
                    },
                    tx_msg_done_timestamp: data.tx_msg_done_timestamp,
                    start_msg_timestamp: data.start_msg_timestamp,
                    data: unsafe {
                        let ptr = data.p_data_bytes;
                        let len = data.num_data_bytes as usize;
                        let slice = if ptr.is_null() || len == 0 {
                            &[]
                        } else {
                            slice::from_raw_parts(ptr, len)
                        };
                        slice.to_vec().into()
                    },
                    extra_info_header: extra_header.take(),
                    extra_info_footer: extra_footer.take(),
                }
                .into()
            }
            PduIt::Error => {
                let data = unsafe { &*(item.p_data as *const ErrorData) };
                PduErrorEvent {
                    code: data.error_code_id,
                    extra_code: data.extra_error_info_id,
                }
                .into()
            }
            PduIt::Info => {
                let data = unsafe { &*(item.p_data as *const InfoData) };
                PduInfoEvent {
                    code: data.info_code,
                    extra_code: data.extra_info_data,
                }
                .into()
            }
            typ => {
                self.pdu_destroy_item(item_ptr as _)?;
                error!(
                    func = FUNC,
                    "Unexpected PduEventItemType = {}. Emulation of PduError::FctFailed...",
                    typ.as_str()
                );
                return Err(PduError::FctFailed)?;
            }
        };

        let h_cop = (item.h_cop != PDU_HANDLE_UNDEF).then(|| item.h_cop);
        let cop_tag = NonZeroUsize::new(item.p_cop_tag as _);

        self.pdu_destroy_item(item_ptr as _)?;

        Ok(Some(PduEvent {
            target: target.clone(),
            h_cop,
            cop_tag,
            timestamp: item.timestamp,
            data,
        }))
    }

    pub fn pdu_get_version(&self, h_mod: PduModuleHandle) -> ApiResult<PduVersionData> {
        const FUNC: &'static str = "PDUGetVersion";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, "D-PDU API Call Args");

        let mut version_data = VersionData::default();

        let get_version_fn = self.get_pdu_function::<PduGetVersionFn>(FUNC.as_bytes())?;
        let result = get_version_fn(h_mod, &mut version_data);

        trace!(
            func = FUNC,
            version_data_ptr = format!("0x{:x}", &version_data as *const VersionData as usize),
            "D-PDU API Call Return"
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        let version_data = PduVersionData {
            mvci_part1_standard_version: version_data.mvci_part1_standard_version.into(),
            mvci_part2_standard_version: version_data.mvci_part2_standard_version.into(),
            hw_serial_number: version_data.hw_serial_number,
            hw_name: c_str(version_data.hw_name.as_ptr() as _),
            hw_version: version_data.hw_version.into(),
            hw_date: version_data.hw_date.into(),
            hw_interface: version_data.hw_interface,
            fw_name: c_str(version_data.fw_name.as_ptr() as _),
            fw_version: version_data.fw_version.into(),
            fw_date: version_data.fw_date.into(),
            vendor_name: c_str(version_data.vendor_name.as_ptr() as _),
            pdu_api_sw_name: c_str(version_data.pdu_api_sw_name.as_ptr() as _),
            pdu_api_sw_version: version_data.pdu_api_sw_version.into(),
            pdu_api_sw_date: version_data.pdu_api_sw_date.into(),
        };

        Ok(version_data)
    }

    pub fn pdu_get_object_id(&self, object: PduObjt, short_name: &str) -> ApiResult<PduObjectId> {
        const FUNC: &'static str = "PDUGetObjectId";
        self.log_api_call(FUNC);

        trace!(
            func = FUNC,
            object = object.as_str(),
            short_name,
            "D-PDU API Call Args"
        );

        if let Some(desc) = &self.module_description {
            // First, we will try to obtain the required object ID from the module description
            // file supplied with the D-PDU API driver in order to reduce
            // the number of D-PDU API calls.
            let object_id = match object {
                PduObjt::IoCtrl => desc.io_controls.get_by_short_name(short_name).map(|v| v.id),
                PduObjt::Resource => desc.resources.get_by_short_name(short_name).map(|v| v.id),
                PduObjt::Protocol => desc.protocols.get_by_short_name(short_name).map(|v| v.id),
                PduObjt::BusType => desc.bus_types.get_by_short_name(short_name).map(|v| v.id),
                PduObjt::PinType => desc.pin_types.get_by_short_name(short_name).map(|v| v.id),
                PduObjt::ComParam => desc.com_params.get_by_short_name(short_name).map(|v| v.id),
            };

            if let Some(id) = object_id {
                return Ok(id);
            }
        }

        let short_name = CString::new(short_name).expect("CString::new() failed");
        let mut object_id: MaybeUninit<u32> = MaybeUninit::uninit();

        let get_object_id_fn = self.get_pdu_function::<PduGetObjectIdFn>(FUNC.as_bytes())?;
        let result = get_object_id_fn(object, short_name.as_ptr() as _, object_id.as_mut_ptr());

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        // SAFETY:
        // PDUGetObjectId guarantees that `object_id` is initialized on success.
        let object_id = unsafe { object_id.assume_init() };

        trace!(func = FUNC, object_id, "D-PDU API Call Return");

        Ok(object_id)
    }

    pub fn pdu_get_com_param(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        object_id: PduObjectIdSource,
    ) -> ApiResult<PduComParam> {
        const FUNC: &'static str = "PDUGetComParam";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, h_cll, %object_id, "D-PDU API Call Args");

        let id = match &object_id {
            PduObjectIdSource::Id(v) => *v,
            PduObjectIdSource::ShortName(v) => {
                let id = self.pdu_get_object_id(PduObjt::ComParam, &v)?;
                if id == PDU_ID_UNDEF {
                    warn!(com_param = v, "ComParam not supported");
                    // This is not a critical error.
                    // Therefore, we will not log it separately.
                    return Err(PduError::ComParamNotSupported)?;
                }
                id
            }
        };

        let mut item_ptr: *mut ParamItem = ptr::null_mut();
        let get_com_param_fn = self.get_pdu_function::<PduGetComParamFn>(FUNC.as_bytes())?;
        let result = get_com_param_fn(h_mod, h_cll, id, &mut item_ptr);

        trace!(
            func = FUNC,
            item_ptr = format!("0x{:x}", item_ptr as usize),
            item_type = ?NonNull::new(item_ptr).map(|wptr| unsafe { (&*wptr.as_ptr()).item_type }),
            "D-PDU API Call Return"
        );

        if !result.is_success() {
            return match result {
                PduError::ComParamNotSupported | PduError::InvalidParameters => {
                    // Some drivers return InvalidParameters instead of ComParamNotSupported.
                    warn!(com_param = %object_id, "ComParam not supported");
                    // This is not a critical error.
                    // Therefore, we will not log it separately.
                    Err(PduError::ComParamNotSupported)?
                }
                _ => {
                    self.log_failed_api_call(FUNC, result);
                    Err(result)?
                }
            };
        }

        if item_ptr.is_null() {
            error!(
                func = FUNC,
                "Item pointer is null. Emulation of PduError::FctFailed..."
            );
            return Err(PduError::FctFailed)?;
        }

        let cp = unsafe {
            use ptr::read;

            let item = &*item_ptr;
            let data_ptr = item.p_com_param_data;
            let short_name = OnceCell::new();

            match &object_id {
                PduObjectIdSource::ShortName(v) => {
                    let _ = short_name.set(v.clone());
                }
                _ => {
                    let sn_opt = self
                        .module_description
                        .as_ref()
                        .and_then(|mdf_desc| mdf_desc.com_params.get_by_id(id))
                        .and_then(|mdf_cp| mdf_cp.short_name.clone());

                    if let Some(sn) = sn_opt {
                        let _ = short_name.set(sn);
                    }
                }
            };

            PduComParam {
                short_name,
                id,
                class: item.com_param_class,
                variant: match item.com_param_data_type {
                    PduPt::Unum8 => CpVariant::Unum8(read(data_ptr as _)),
                    PduPt::Snum8 => CpVariant::Snum8(read(data_ptr as _)),
                    PduPt::Unum16 => CpVariant::Unum16(read(data_ptr as _)),
                    PduPt::Snum16 => CpVariant::Snum16(read(data_ptr as _)),
                    PduPt::Unum32 => CpVariant::Unum32(read(data_ptr as _)),
                    PduPt::Snum32 => CpVariant::Snum32(read(data_ptr as _)),
                    PduPt::ByteField => CpVariant::ByteField({
                        let data = &*(data_ptr as *const ParamByteFieldData);
                        let ptr = data.p_data_array;
                        let len = data.param_act_len as _;
                        let slice = if ptr.is_null() || len == 0 {
                            &[]
                        } else {
                            slice::from_raw_parts(ptr, len)
                        };
                        ByteFieldComParam::new(
                            slice.to_vec(),
                            Some(data.param_max_len as _),
                        )
                    }),
                    PduPt::StructField => CpVariant::StructField({
                        let data = &*(data_ptr as *const ParamStructFieldData);
                        let ptr = data.p_struct_array as *mut StructComParam;
                        let len = data.param_act_entries as _;
                        let slice = if ptr.is_null() || len == 0 {
                            &[]
                        } else {
                            slice::from_raw_parts(ptr, len)
                        };
                        StructFieldComParam::new(
                            data.com_param_struct_type,
                            slice.to_vec(),
                            Some(data.param_max_entries as _),
                        )
                    }),
                    PduPt::LongField => CpVariant::LongField({
                        let data = &*(data_ptr as *const ParamLongFieldData);
                        let ptr = data.p_data_array;
                        let len = data.param_act_len as _;
                        let slice = if ptr.is_null() || len == 0 {
                            &[]
                        } else {
                            slice::from_raw_parts(ptr, len)
                        };
                        LongFieldComParam::new(
                            slice.to_vec(),
                            Some(data.param_max_len as _),
                        )
                    }),
                },
            }
        };

        self.pdu_destroy_item(item_ptr as _)?;

        Ok(cp)
    }

    pub fn pdu_set_com_param(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        cp: &PduComParam,
    ) -> ApiResult<()> {
        const FUNC: &'static str = "PDUSetComParam";
        self.log_api_call(FUNC);

        if matches!(cp.class, PduPc::UniqueId) {
            // Chapter 9.4.27.1:
            //
            // NOTE ComParams that are of type PDU_PC_UNIQUE_ID can only be used with
            // the Unique Response ID Table.
            // They cannot be used in the functions PDUGetComParam() or PDUSetComParam().
            //
            // Therefore, to reduce the number of calls to the D-PDU API, we proactively
            // return the PduError::InvalidParameters error on our side.
            let result = PduError::InvalidParameters;
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        cp.try_init_short_name(self);

        let item = ParamItem {
            item_type: PduIt::Param,
            com_param_id: cp.id,
            com_param_data_type: cp.variant.get_pdu_type(),
            com_param_class: cp.class,
            p_com_param_data: cp.variant.get_pdu_ptr().as_ptr() as _,
        };

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            com_param = cp.get_debug_name(),
            com_param_ptr = format!("0x{:x}", &item as *const _ as usize),
            "D-PDU API Call Args"
        );

        let set_com_param_fn = self.get_pdu_function::<PduSetComParamFn>(FUNC.as_bytes())?;
        let result = set_com_param_fn(h_mod, h_cll, &item as *const _ as _);

        if !result.is_success() {
            return match result {
                PduError::ComParamNotSupported | PduError::InvalidParameters => {
                    // Some drivers return InvalidParameters instead of ComParamNotSupported.
                    warn!(com_param = cp.get_debug_name(), "ComParam not supported");
                    // This is not a critical error.
                    // Therefore, we will not log it separately.
                    Err(PduError::ComParamNotSupported)?
                }
                _ => {
                    self.log_failed_api_call(FUNC, result);
                    Err(result)?
                }
            };
        }

        Ok(())
    }

    pub fn pdu_set_unique_resp_id_table(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        table: &PduComParamTable,
    ) -> ApiResult<()> {
        const FUNC: &'static str = "PDUSetUniqueRespIdTable";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, h_cll, "D-PDU API Call Args");

        type EphemeralGroupKey = usize;

        let mut temp_com_param_groups: HashMap<EphemeralGroupKey, Vec<ParamItem>> =
            HashMap::with_capacity(table.len());
        let mut temp_unique_groups: Vec<EcuUniqueRespData> = Vec::with_capacity(table.len());

        for (key, (unique_resp_identifier, group_set)) in table.iter().enumerate() {
            let mut com_param_set = Vec::with_capacity(group_set.len());

            trace!(
                func = FUNC,
                group = key,
                unique_resp_identifier,
                "D-PDU API Call Args"
            );

            for cp in group_set.iter() {
                if !matches!(cp.class, PduPc::UniqueId) {
                    error!(
                        com_param = cp.get_debug_name(),
                        class = cp.class.as_str(),
                        "Invalid class of the PduComParam stored in PduComParamTable"
                    );
                    let result = PduError::InvalidParameters;
                    self.log_failed_api_call(FUNC, result);
                    return Err(result)?;
                }

                cp.try_init_short_name(self);

                let item = ParamItem {
                    item_type: PduIt::Param,
                    com_param_id: cp.id,
                    com_param_data_type: cp.variant.get_pdu_type(),
                    com_param_class: PduPc::UniqueId,
                    p_com_param_data: cp.variant.get_pdu_ptr().as_ptr() as _,
                };

                trace!(
                    func = FUNC,
                    group = key,
                    com_param = cp.get_debug_name(),
                    com_param_ptr = format!("{:#x}", &item as *const _ as usize),
                    "D-PDU API Call Args"
                );

                com_param_set.push(item);
            }

            temp_com_param_groups.insert(key, com_param_set);

            let temp_com_param_group = temp_com_param_groups.get(&key).expect("inserted above");

            temp_unique_groups.push(EcuUniqueRespData {
                unique_resp_identifier: *unique_resp_identifier,
                num_param_items: temp_com_param_group.len() as _,
                p_params: temp_com_param_group.as_ptr() as _,
            });
        }

        let table = UniqueRespIdTableItem {
            item_type: PduIt::UniqueRespIdTable,
            num_entries: temp_unique_groups.len() as _,
            p_unique_data: temp_unique_groups.as_ptr() as _,
        };

        trace!(
            func = FUNC,
            table_num_entries = table.num_entries,
            table_ptr = format!("{:#x}", &table as *const _ as usize),
            "D-PDU API Call Args"
        );

        let set_unique_resp_id_table_fn =
            self.get_pdu_function::<PduSetUniqueRespIdTableFn>(FUNC.as_bytes())?;
        let result = set_unique_resp_id_table_fn(h_mod, h_cll, &table as *const _ as _);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_register_event_callback(
        &self,
        target: &PduEventTarget,
        callback: Option<EventCallbackFn>,
    ) -> ApiResult<()> {
        const FUNC: &'static str = "PDURegisterEventCallback";
        self.log_api_call(FUNC);

        trace!(func = FUNC, %target, "D-PDU API Call Args");

        let (h_mod, h_cll) = match target {
            PduEventTarget::Module(h_mod) => {
                if h_mod == &PDU_HANDLE_UNDEF {
                    let result = PduError::InvalidHandle;
                    error!(
                        func = FUNC,
                        "Module handle of the PduEventCallbackTarget cannot be PDU_HANDLE_UNDEF"
                    );
                    self.log_failed_api_call(FUNC, result);
                    return Err(result)?;
                }

                (h_mod.to_owned(), PDU_HANDLE_UNDEF)
            }
            PduEventTarget::ComLogicalLink(h_mod, h_cll) => {
                if h_mod == &PDU_HANDLE_UNDEF {
                    let result = PduError::InvalidHandle;
                    error!(
                        func = FUNC,
                        "Module handle of the PduEventCallbackTarget cannot be PDU_HANDLE_UNDEF"
                    );
                    self.log_failed_api_call(FUNC, result);
                    return Err(result)?;
                } else if h_cll == &PDU_HANDLE_UNDEF {
                    let result = PduError::InvalidHandle;
                    error!(
                        func = FUNC,
                        "ComLogicalLink handle of the PduEventCallbackTarget cannot be PDU_HANDLE_UNDEF"
                    );
                    self.log_failed_api_call(FUNC, result);
                    return Err(result)?;
                }

                (h_mod.to_owned(), h_cll.to_owned())
            }
            PduEventTarget::System => (PDU_HANDLE_UNDEF, PDU_HANDLE_UNDEF),
        };

        trace!(func = FUNC, h_mod, h_cll, "D-PDU API Call Args");

        let register_event_callback_fn =
            self.get_pdu_function::<PduRegisterCallbackFn>(FUNC.as_bytes())?;

        let result =
            register_event_callback_fn(h_mod, h_cll, unsafe { std::mem::transmute(callback) });

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_start_com_primitive(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        cop_type: PduCopt,
        data: &[u8],
        params: Option<&PduComPrimitiveParams>,
        tag: Option<PduUniqueCopTag>
    ) -> ApiResult<PduCopData> {
        const FUNC: &'static str = "PDUStartComPrimitive";
        self.log_api_call(FUNC);

        let tag = tag
            .map(|v| v.get())
            .unwrap_or_default();

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            cop_type = cop_type.as_str(),
            tag = format!("{tag:#x}"),
            "D-PDU API Call Args"
        );

        let mut cop_handle: MaybeUninit<PduCopHandle> = MaybeUninit::uninit();
        let start_com_primivite_fn =
            self.get_pdu_function::<PduStartComPrimitiveFn>(FUNC.as_bytes())?;

        let result = match cop_type {
            PduCopt::UpdateParam | PduCopt::RestoreParam => {
                if data.len() != 0 {
                    warn!(
                        func = FUNC,
                        "when PduCopt = UpdateParam or RestoreParam, data is not required"
                    );
                }
                if params.is_some() {
                    warn!(
                        func = FUNC,
                        "when PduCopt = UpdateParam or RestoreParam, PduComPrimitiveParams is not required"
                    );
                }

                trace!(
                    func = FUNC,
                    data_len = 0,
                    data_ptr = "<nullptr>",
                    cop_ctrl_data_ptr = "<nullptr>",
                    tag,
                    cop_handle_ptr = format!("{:#x}", cop_handle.as_ptr() as usize),
                    "D-PDU API Call Args"
                );

                start_com_primivite_fn(
                    h_mod,
                    h_cll,
                    cop_type,
                    0,               // data len
                    ptr::null_mut(), // data ptr
                    ptr::null_mut(), // cop ctrl data
                    tag as *mut _, // tag
                    cop_handle.as_mut_ptr(),
                )
            }
            _ => {
                let params = params.expect(&format!(
                    "when PduCopt = {}, PduComPrimitiveParams is required",
                    cop_type.as_str()
                ));

                let flags = params.tx_flag.get_pdu_flag_data();

                trace!(
                    func = FUNC,

                    cop_delay_ms = params.time,
                    send_cycles = params.send_cycles.to_i32(),
                    receive_cycles = params.receive_cycles.to_i32(),
                    buffer = params.temp_param_update.as_str(),

                    flags_ptr = format!("{:#x}", flags.as_ptr() as usize),
                    flags = ?flags,

                    expected_responses_len = params.expected_responses.len(),

                    "D-PDU API Call Args"
                );

                let expected_responses = params.expected_responses
                    .iter()
                    .map(|v| {
                        trace!(
                            func = FUNC,

                            response_type = v.response_type.as_str(),
                            acceptance_id = v.acceptance_id,

                            mask_data_ptr = format!("{:#x}", v.mask_data.get_mask().as_ptr() as usize),
                            mask_data_len = v.mask_data.mask.len(),
                            mask_data = ?v.mask_data.mask,

                            mask_pattern_ptr = format!("{:#x}", v.mask_data.get_pattern().as_ptr() as usize),
                            mask_pattern_len = v.mask_data.pattern.len(),
                            mask_pattern = ?v.mask_data.pattern,

                            unique_response_ids_ptr = format!("{:#x}", v.unique_response_ids.as_ptr() as usize),
                            unique_response_ids_len = v.unique_response_ids.len(),
                            unique_response_ids = ?v.unique_response_ids,

                            "D-PDU API Call Args"
                        );

                        ExpRespData {
                            response_type: v.response_type as _,
                            acceptance_id: v.acceptance_id,
                            num_mask_pattern_bytes: v.mask_data.len() as _,
                            p_mask_data: v.mask_data.get_mask().as_ptr() as _,
                            p_pattern_data: v.mask_data.get_pattern().as_ptr() as _,
                            num_unique_resp_ids: v.unique_response_ids.len() as _,
                            p_unique_resp_ids: v.unique_response_ids.as_ptr() as _,
                        }
                    })
                    .collect::<Vec<_>>();

                let cop_ctrl_data = CopCtrlData {
                    time: params.time,
                    num_send_cycles: params.send_cycles.to_i32(),
                    num_receive_cycles: params.receive_cycles.to_i32(),
                    temp_param_update: params.temp_param_update as _,
                    tx_flag: FlagData {
                        num_flag_bytes: flags.len() as _,
                        p_flag_data: flags.as_ptr() as _,
                    },
                    num_possible_expected_responses: expected_responses.len() as _,
                    expected_response_array: expected_responses.as_ptr() as _,
                };

                trace!(
                    func = FUNC,
                    data_len = data.len(),
                    data_ptr = format!("{:#x}", data.as_ptr() as usize),
                    cop_ctrl_data_ptr = format!("{:#x}", &cop_ctrl_data as *const _ as usize),
                    tag,
                    cop_handle_ptr = format!("{:#x}", cop_handle.as_ptr() as usize),
                    "D-PDU API Call Args"
                );

                start_com_primivite_fn(
                    h_mod,
                    h_cll,
                    cop_type,
                    data.len() as _,
                    data.as_ptr() as _,
                    &cop_ctrl_data as *const _ as _,
                    tag as *mut _, // tag
                    cop_handle.as_mut_ptr(),
                )
            }
        };

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        // SAFETY:
        // PDUStartComPrimitive guarantees that `phCoP` is initialized on success.
        let cop_handle = unsafe { cop_handle.assume_init() };

        trace!(func = FUNC, cop_handle, "D-PDU API Call Return");

        Ok(PduCopData {
            h_cop: cop_handle,
            cop_type,
        })
    }

    ///| IOCTL Short Name                      | Target | Input Data Type               | Output Data Type            | Purpose
    ///|---------------------------------------|--------|-------------------------------|-----------------------------|--------------------------------------------------------------------------------
    ///| PDU_IOCTL_RESET                       | M      | —                             | —                           | Reset specific MVCI protocol module.
    ///| PDU_IOCTL_CLEAR_TX_QUEUE              | L      | —                             | —                           | Clear transmit queue of specific ComLogicalLink.
    ///| PDU_IOCTL_SUSPEND_TX_QUEUE            | L      | —                             | —                           | Suspend transmit queue of specific ComLogicalLink.
    ///| PDU_IOCTL_RESUME_TX_QUEUE             | L      | —                             | —                           | Resume transmit queue of specific ComLogicalLink.
    ///| PDU_IOCTL_CLEAR_RX_QUEUE              | L      | —                             | —                           | Clear event queue of specific ComLogicalLink.
    ///| PDU_IOCTL_READ_VBATT                  | M      | —                             | PDU_IT_IO_UNUM32            | Read voltage on pin 16 of MVCI protocol module.
    ///| PDU_IOCTL_SET_PROG_VOLTAGE            | M      | PDU_IT_IO_PROG_VOLTAGE        | —                           | Set programmable voltage on DLC connector pin/resource.
    ///| PDU_IOCTL_READ_PROG_VOLTAGE           | M      | —                             | PDU_IT_IO_UNUM32            | Read feedback of programmable voltage.
    ///| PDU_IOCTL_GENERIC                     | M      | PDU_IT_IO_BYTE_ARRAY          | —                           | Send a generic message to MVCI protocol module drivers.
    ///| PDU_IOCTL_SET_BUFFER_SIZE             | L      | PDU_IT_IO_UNUM32              | —                           | Set buffer size limit of item.
    ///| PDU_IOCTL_START_MSG_FILTER            | L      | PDU_IT_IO_FILTER              | —                           | Start filtering incoming messages for specified ComLogicalLink.
    ///| PDU_IOCTL_CLEAR_MSG_FILTER            | L      | —                             | —                           | Clear all message filters for the ComLogicalLink.
    ///| PDU_IOCTL_STOP_MSG_FILTER             | L      | PDU_IT_IO_UNUM32              | —                           | Stop specified filter based on filter number.
    ///| PDU_IOCTL_SET_EVENT_QUEUE_PROPERTIES  | L      | PDU_IT_IO_EVENT_QUEUE_PROPERTY| —                           | Set size and mode of ComLogicalLink event queue.
    ///| PDU_IOCTL_GET_CABLE_ID                | M      | —                             | PDU_IT_IO_UNUM32            | Get cable ID connected to MVCI protocol module.
    ///| PDU_IOCTL_SEND_BREAK                  | L      | —                             | —                           | Send UART Break Signal on ComLogicalLink.
    ///| PDU_IOCTL_READ_IGNITION_SENSE_STATE   | M      | PDU_IT_IO_UNUM32              | PDU_IT_IO_UNUM32            | Read ignition sense state from vehicle connector pin.
    ///| PDU_IOCTL_VEHICLE_ID_REQUEST          | S, M   | PDU_IT_IO_VEHICLE_ID_REQUEST  | —                           | Send vehicle identification request (DoIP).
    ///| PDU_IOCTL_SET_ETH_SWITCH_STATE        | M      | PDU_IT_IO_ETH_SWITCH_STATE    | —                           | Switch Ethernet activation PIN on DLC.
    ///| PDU_IOCTL_GET_ENTITY_STATUS           | M      | PDU_IT_IO_ENTITY_ADDRESS      | PDU_IT_IO_ENTITY_STATUS     | Retrieve status of a DoIP entity.
    ///| PDU_IOCTL_GET_DIAGNOSTIC_POWER_MODE   | M      | PDU_IT_IO_ENTITY_ADDRESS      | PDU_IT_IO_UNUM32            | Retrieve diagnostic power mode of a DoIP entity.
    ///| PDU_IOCTL_GET_ETH_PIN_OPTION          | M      | PDU_IT_IO_UNUM32              | PDU_IT_IO_UNUM32            | Determine Ethernet pinout option from Ethernet activation PIN (DLC).
    ///| PDU_IOCTL_TLS_SET_CERTIFICATE         | M      | PDU_IT_IO_TLS_CERTIFICATE     | —                           | Set X.509 certificate(s) used for ECU verification during TLS handshake.
    ///| PDU_IOCTL_TLS_GET_CURRENT_SESSION_MODE| L      | —                             | PDU_IT_IO_UNUM32            | Get current DoIP connection mode (unsecure or secured via TLS).
    ///| PDU_IOCTL_ISOBUS_GET_DETECTED_CFS     | L      | —                             | PDU_IT_IO_BYTEARRAY         | Get list of ISOBUS CF-NAMEs detected on CAN bus (8-byte NAME + 1-byte address).
    pub fn pdu_io_ctl(
        &self,
        target: &PduIoCtlTarget,
        command: &PduIoCtlCommand,
        data: Option<&PduIoCtlData>,
    ) -> ApiResult<Option<PduIoCtlData>> {
        const FUNC: &'static str = "PDUIoCtl";
        self.log_api_call(FUNC);

        let h_mod = target.get_module_handle().unwrap_or(PDU_HANDLE_UNDEF);
        let h_cll = target.get_cll_handle().unwrap_or(PDU_HANDLE_UNDEF);

        trace!(
            func = FUNC,
            ?h_mod,
            ?h_cll,
            %command,
            "D-PDU API Call Args"
        );

        data.inspect(|data| match data {
            PduIoCtlData::U32(v) => trace!(
                func = FUNC,
                data_type = data.as_str(),
                data_u32 = v,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::ProgVoltage(v) => trace!(
                func = FUNC,
                data_type = data.as_str(),
                data_prog_voltage_mv = v.prog_voltage_mv,
                data_pin_on_dlc = v.pin_on_dlc,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::ByteArray(v) => trace!(
                func = FUNC,
                data_type = data.as_str(),
                data_len = v.len(),
                data_value = ?v,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::Filter(v) => trace!(
                func = FUNC,
                data_type = data.as_str(),
                data_filter_type = v.filter_type.as_str(),
                data_filter_number = v.filter_number,
                data_filter_compare_size = v.filter_compare_size,
                data_filter_mask_msg = ?v.filter_mask_msg,
                data_filter_pattern_msg = ?v.filter_pattern_msg,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::EventQueueProperty(v) => trace!(
                func = FUNC,
                data_type = data.as_str(),
                data_queue_size = v.queue_size,
                data_queue_mode = v.queue_mode.as_str(),
                "D-PDU API Call Args"
            ),
        });

        let object_id = match command {
            PduIoCtlCommand::Id(v) => v.to_owned(),
            PduIoCtlCommand::Name(v) => self.pdu_get_object_id(PduObjt::IoCtrl, &v)?,
        };

        let input_data_ptr: *const c_void = data
            .as_ref()
            .map(|v| v.to_pdu_data_item().p_data as _)
            .unwrap_or(ptr::null());

        let mut output_data_ptr = ptr::null_mut();

        trace!(
            func = FUNC,
            input_data_ptr = format!("{:#x}", input_data_ptr as usize),
            output_data_ptr = format!("{:#x}", &output_data_ptr as *const _ as usize),
            "D-PDU API Call Args"
        );

        let io_ctl_fn = self.get_pdu_function::<PduIoctlFn>(FUNC.as_bytes())?;
        let result = io_ctl_fn(
            h_mod,
            h_cll,
            object_id,
            input_data_ptr as _,
            &mut output_data_ptr,
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        if !output_data_ptr.is_null() {
            let data = unsafe { &*output_data_ptr };
            let io_ctl_data: ApiResult<Option<PduIoCtlData>> = unsafe {
                match data.item_type {
                    PduIt::IoUnum32 => Ok(Some(data.p_data.cast::<u32>().read().into())),
                    PduIt::IoProgVoltage => {
                        Ok(Some(data.p_data.cast::<IoProgVoltageData>().read().into()))
                    }
                    PduIt::IoByteArray => {
                        let byte_array = &*data.p_data.cast::<IoByteArrayData>();
                        if byte_array.p_data.is_null() {
                            error!(
                                func = FUNC,
                                data_type = PduIt::IoByteArray.as_str(),
                                "Byte array pointer is null. Emulation of PduError::FctFailed..."
                            );
                            return Err(PduError::FctFailed)?;
                        } else {
                            let ptr = byte_array.p_data;
                            let len = byte_array.data_size as _;
                            let slice = if ptr.is_null() || len == 0 {
                                &[]
                            } else {
                                slice::from_raw_parts(ptr, len)
                            };
                            Ok(Some(IoCtlByteArray(slice.to_vec()).into()))
                        }
                    }
                    PduIt::IoFilter => Ok(Some(data.p_data.cast::<IoFilterData>().read().into())),
                    PduIt::IoEventQueueProperty => Ok(Some(
                        data.p_data.cast::<IoEventQueuePropertyData>().read().into(),
                    )),
                    v => {
                        error!(
                            func = FUNC,
                            data_type = v.as_str(),
                            "Unexpected output data type. Emulation of PduError::FctFailed..."
                        );
                        return Err(PduError::FctFailed)?;
                    }
                }
            };

            self.pdu_destroy_item(output_data_ptr as _)?;

            io_ctl_data
        } else {
            Ok(None)
        }
    }

    pub fn pdu_get_module_ids(&self) -> ApiResult<PduModuleList> {
        const FUNC: &'static str = "PDUGetModuleIds";
        self.log_api_call(FUNC);

        let mut module_list_item_ptr = ptr::null_mut();

        let get_module_ids_fn = self.get_pdu_function::<PduGetModuleIdsFn>(FUNC.as_bytes())?;
        let result = get_module_ids_fn(&mut module_list_item_ptr);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        trace!(
            func = FUNC,
            item_ptr = format!("0x{:x}", module_list_item_ptr as usize),
            item_type = ?NonNull::new(module_list_item_ptr).map(|wptr| unsafe { (&*wptr.as_ptr()).item_type }),
            "D-PDU API Call Return"
        );

        if module_list_item_ptr.is_null() {
            error!(
                func = FUNC,
                "Module list pointer is null. Emulation of PduError::FctFailed..."
            );
            return Err(PduError::FctFailed)?;
        }

        let module_list_item = unsafe { &*module_list_item_ptr };

        let ptr = module_list_item.p_module_data;
        let len = module_list_item.num_entries as _;

        trace!(
            func = FUNC,
            modules_ptr = format!("{:#x}", ptr as usize),
            modules_len = len,
            "D-PDU API Call Return"
        );

        let modules = if ptr.is_null() || len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(ptr, len) }
        };

        let module_list = modules
            .into_iter()
            .map(|v| {
                let vendor_module_name = c_str(v.vendor_module_name as _);
                let vendor_additional_info = c_str(v.vendor_additional_info as _);

                trace!(
                    func = FUNC,
                    module_handle = v.h_mod,
                    module_type_id = v.module_type_id,
                    module_name = vendor_module_name,
                    module_add_info = vendor_additional_info,
                    "D-PDU API Call Return"
                );

                PduModuleData {
                    h_mod: v.h_mod,
                    module_type_id: v.module_type_id,
                    vendor_module_name,
                    vendor_additional_info,
                    status: v.status,
                }
            })
            .collect::<Vec<_>>();

        self.pdu_destroy_item(module_list_item_ptr as _)?;

        Ok(module_list)
    }

    pub fn pdu_get_status(&self, target: &PduStatusTarget) -> ApiResult<PduStatusData> {
        const FUNC: &'static str = "PDUGetStatus";
        self.log_api_call(FUNC);

        let h_mod = target.get_module_handle().unwrap_or(PDU_HANDLE_UNDEF);
        let h_cll = target.get_cll_handle().unwrap_or(PDU_HANDLE_UNDEF);
        let h_cop = target.get_cop_handle().unwrap_or(PDU_HANDLE_UNDEF);

        trace!(func = FUNC, h_mod, h_cll, h_cop, "D-PDU API Call Args");

        let mut status_code: MaybeUninit<u32> = MaybeUninit::uninit();
        let mut timestamp: MaybeUninit<u32> = MaybeUninit::uninit();
        let mut extra_info: MaybeUninit<u32> = MaybeUninit::uninit();

        let get_status_fn = self.get_pdu_function::<PduGetStatusFn>(FUNC.as_bytes())?;
        let result = get_status_fn(
            h_mod,
            h_cll,
            h_cop,
            status_code.as_mut_ptr() as _,
            timestamp.as_mut_ptr(),
            extra_info.as_mut_ptr(),
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        let status_code = unsafe { status_code.assume_init() };
        let timestamp = unsafe { timestamp.assume_init() };
        let extra_info = unsafe { extra_info.assume_init() };

        trace!(
            func = FUNC,
            status_code, timestamp, extra_info, "D-PDU API Call Return"
        );

        let status_code = match PduStatus::try_from(status_code) {
            Ok(v) => v,
            Err(_) => {
                error!(
                    func = FUNC,
                    "Received out-of-bounds PduStatus value: {:#x}. Emulation of PduError::FctFailed...",
                    status_code,
                );
                return Err(PduError::FctFailed)?;
            }
        };

        Ok(PduStatusData {
            target: target.clone(),
            status_code,
            timestamp,
            extra_info,
        })
    }

    pub fn pdu_create_com_logical_link(
        &self,
        h_mod: PduModuleHandle,
        create_type: &CllCreateType,
        create_flags: &CllCreateFlags,
        tag: Option<PduUniqueCllTag>,
    ) -> ApiResult<PduCllData> {
        const FUNC: &'static str = "PDUCreateComLogicalLink";
        self.log_api_call(FUNC);

        let tag = tag
            .map(|v| v.get())
            .unwrap_or_default();

        trace!(func = FUNC, h_mod, tag = format!("{tag:#x}"), "D-PDU API Call Args");

        let flag_bytes = create_flags.get_pdu_flag_data();
        let flag_data = FlagData {
            num_flag_bytes: flag_bytes.len() as _,
            p_flag_data: flag_bytes.as_ptr() as _,
        };

        let mut cll_handle: MaybeUninit<PduCllHandle> = MaybeUninit::uninit();

        let create_com_logical_link_fn =
            self.get_pdu_function::<PduCreateComLogicalLinkFn>(FUNC.as_bytes())?;
        let result = match &create_type {
            CllCreateType::ResourceId(v) => {
                trace!(func = FUNC, resource_id = v, "D-PDU API Call Args");
                create_com_logical_link_fn(
                    h_mod,
                    ptr::null_mut(),
                    v.clone(),
                    tag as *mut _,
                    cll_handle.as_mut_ptr(),
                    &flag_data as *const FlagData as _,
                )
            }
            CllCreateType::ResourceData {
                bus,
                protocol,
                pins,
            } => {
                trace!(func = FUNC, %bus, %protocol, "D-PDU API Call Args");

                let bus_type_id = bus.resolve_bus_id(self)?;
                let protocol_id = protocol.resolve_protocol_id(self)?;

                let pin_data = target_pins_to_pin_data(self, FUNC, &pins)?;

                let rsc_data = RscData {
                    bus_type_id,
                    protocol_id,
                    num_pin_data: pin_data.len() as _,
                    p_dlc_pin_data: pin_data.as_ptr() as _,
                };

                trace!(
                    func = FUNC,
                    rsc_data_ptr = format!("{:#x}", &rsc_data as *const _ as usize),
                    bus_type_id,
                    protocol_id,
                    pin_len = pin_data.len(),
                    "D-PDU API Call Args"
                );

                create_com_logical_link_fn(
                    h_mod,
                    &rsc_data as *const RscData as _,
                    PDU_ID_UNDEF,
                    tag as *mut _,
                    cll_handle.as_mut_ptr(),
                    &flag_data as *const FlagData as _,
                )
            }
        };

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        let h_cll = unsafe { cll_handle.assume_init() };

        trace!(func = FUNC, h_cll, "D-PDU API Call Return");

        Ok(PduCllData {
            h_mod,
            h_cll: unsafe { cll_handle.assume_init() },
            create_type: create_type.clone(),
            create_flags: create_flags.clone(),
        })
    }

    pub fn pdu_destroy_com_logical_link(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
    ) -> ApiResult<()> {
        const FUNC: &'static str = "PDUDestroyComLogicalLink";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, h_cll, "D-PDU API Call Args");

        let destroy_fn =
            self.get_pdu_function::<PduDestroyComLogicalLinkFn>(b"PDUDestroyComLogicalLink")?;

        let result = destroy_fn(h_mod, h_cll);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_get_last_error(&self, target: &PduLastErrorTarget) -> ApiResult<PduErrorData> {
        const FUNC: &'static str = "PDUGetLastError";
        self.log_api_call(FUNC);

        let h_mod = target.get_module_handle().unwrap_or(PDU_HANDLE_UNDEF);
        let h_cll = target.get_cll_handle().unwrap_or(PDU_HANDLE_UNDEF);

        let mut error_code: MaybeUninit<u32> = MaybeUninit::uninit(); // will transform to PduErrorEvt
        let mut h_cop: MaybeUninit<PduCopHandle> = MaybeUninit::uninit(); // maybe undef
        let mut timestamp: MaybeUninit<u32> = MaybeUninit::uninit();
        let mut extra_info_code: MaybeUninit<u32> = MaybeUninit::uninit(); // maybe ID_UNDEF?

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            error_code_ptr = format!("{:#x}", error_code.as_ptr() as usize),
            h_cop_ptr = format!("{:#x}", h_cop.as_ptr() as usize),
            timestamp_ptr = format!("{:#x}", timestamp.as_ptr() as usize),
            extra_info_code_ptr = format!("{:#x}", extra_info_code.as_ptr() as usize),
            "D-PDU API Call Args"
        );

        let get_last_error_fn = self.get_pdu_function::<PduGetLastErrorFn>(FUNC.as_bytes())?;
        let result = get_last_error_fn(
            h_mod,
            h_cll,
            error_code.as_mut_ptr() as _,
            h_cop.as_mut_ptr(),
            timestamp.as_mut_ptr(),
            extra_info_code.as_mut_ptr(),
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        let error_code = unsafe { error_code.assume_init() };
        let h_cop = unsafe { h_cop.assume_init() };
        let timestamp = unsafe { timestamp.assume_init() };
        let extra_info_code = unsafe { extra_info_code.assume_init() };

        trace!(
            func = FUNC,
            error_code, h_cop, timestamp, extra_info_code, "D-PDU API Call Return"
        );

        let error_event = match PduErrorEvt::try_from(error_code) {
            Ok(v) => v,
            Err(_) => {
                error!(
                    func = FUNC,
                    "Received out-of-bounds PduErrorEvt value: {:#x}. Emulation of PduError::FctFailed...",
                    error_code,
                );
                return Err(PduError::FctFailed)?;
            }
        };

        Ok(PduErrorData {
            target: target.clone(),
            error_event,
            h_cop: (h_cop != PDU_HANDLE_UNDEF).then(|| h_cop),
            timestamp,
            extra_info_code: (extra_info_code != PDU_ID_UNDEF).then(|| extra_info_code),
        })
    }

    pub fn pdu_get_resource_status(
        &self,
        resources: Vec<PduResource>,
    ) -> ApiResult<PduResourceStatus> {
        const FUNC: &'static str = "PDUGetResourceStatus";
        self.log_api_call(FUNC);

        let mut map = HashMap::new();

        if resources.len() == 0 {
            warn!(func = FUNC, "Resources is empty");
            return Ok(map);
        }

        let mut raw_resources = resources
            .iter()
            .map(|v| {
                trace!(
                    func = FUNC,
                    resource_h_mod = v.h_mod,
                    resource_id = v.resource_id,
                    "D-PDU API Call Args"
                );

                RscStatusData {
                    h_mod: v.h_mod,
                    resource_id: v.resource_id,
                    resource_status: 0,
                }
            })
            .collect::<Vec<_>>();

        let mut item = RscStatusItem {
            item_type: PduIt::RscStatus,
            num_entries: resources.len() as _,
            p_resource_status_data: raw_resources.as_mut_ptr(),
        };

        trace!(
            func = FUNC,
            item_ptr = format!("{:#x}", &item as *const _ as usize),
            item_len = resources.len(),
            resources_ptr = format!("{:#x}", raw_resources.as_ptr() as usize),
            "D-PDU API Call Args"
        );

        let get_resource_status_fn =
            self.get_pdu_function::<PduGetResourceStatusFn>(FUNC.as_bytes())?;
        let result = get_resource_status_fn(&mut item);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        for resource in resources {
            'il: for raw in raw_resources.iter() {
                if resource.h_mod != raw.h_mod || resource.resource_id != raw.resource_id {
                    continue 'il;
                }

                let status = raw.resource_status;

                let busy = ((status >> 0) & 1).try_into().unwrap(); // SAFE
                let available = ((status >> 1) & 1).try_into().unwrap(); // SAFE
                let transmit_queue_lock = ((status >> 2) & 1).try_into().unwrap(); // SAFE
                let physical_com_param_lock = ((status >> 3) & 1).try_into().unwrap(); // SAFE

                trace!(
                    func = FUNC,
                    resource_h_mod = raw.h_mod,
                    resource_id = raw.resource_id,
                    resource_status = status,
                    busy,
                    available,
                    transmit_queue_lock,
                    physical_com_param_lock,
                    "D-PDU API Call Args"
                );

                map.insert(
                    resource,
                    ResourceStatus {
                        raw_status: status,
                        busy,
                        available,
                        transmit_queue_lock,
                        physical_com_param_lock,
                    },
                );

                break 'il;
            }
        }

        Ok(map)
    }

    pub fn pdu_connect(&self, h_mod: PduModuleHandle, h_cll: PduCllHandle) -> ApiResult<()> {
        const FUNC: &'static str = "PDUConnect";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, h_cll, "D-PDU API Call Args");

        let connect_fn = self.get_pdu_function::<PduConnectFn>(FUNC.as_bytes())?;
        let result = connect_fn(h_mod, h_cll);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_disconnect(&self, h_mod: PduModuleHandle, h_cll: PduCllHandle) -> ApiResult<()> {
        const FUNC: &'static str = "PDUDisconnect";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, h_cll, "D-PDU API Call Args");

        let connect_fn = self.get_pdu_function::<PduDisconnectFn>(FUNC.as_bytes())?;
        let result = connect_fn(h_mod, h_cll);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_lock_resource(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        mask: PduLockResourceMask,
    ) -> ApiResult<()> {
        const FUNC: &'static str = "PDULockResource";
        self.log_api_call(FUNC);

        let mask_data = mask.get_pdu_data();

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            mask = format!("0x{mask_data:#x}"),
            lock_physical_com_params = mask.lock_physical_com_params,
            lock_physical_transmit_queue = mask.lock_physical_transmit_queue,
            "D-PDU API Call Args"
        );

        let lock_resource_fn = self.get_pdu_function::<PduLockResourceFn>(FUNC.as_bytes())?;
        let result = lock_resource_fn(h_mod, h_cll, mask_data);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_unlock_resource(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        mask: PduLockResourceMask,
    ) -> ApiResult<()> {
        const FUNC: &'static str = "PDUUnlockResource";
        self.log_api_call(FUNC);

        let mask_data = mask.get_pdu_data();

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            mask = format!("0x{mask_data:#x}"),
            lock_physical_com_params = mask.lock_physical_com_params,
            lock_physical_transmit_queue = mask.lock_physical_transmit_queue,
            "D-PDU API Call Args"
        );

        let lock_resource_fn = self.get_pdu_function::<PduUnlockResourceFn>(FUNC.as_bytes())?;
        let result = lock_resource_fn(h_mod, h_cll, mask_data);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_module_connect(&self, h_mod: PduModuleHandle) -> ApiResult<()> {
        const FUNC: &'static str = "PDUModuleConnect";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, "D-PDU API Call Args");

        let module_connect_fn = self.get_pdu_function::<PduModuleConnectFn>(FUNC.as_bytes())?;
        let result = module_connect_fn(h_mod);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_module_disconnect(&self, h_mod: Option<PduModuleHandle>) -> ApiResult<()> {
        const FUNC: &'static str = "PDUModuleDisconnect";
        self.log_api_call(FUNC);

        let h_mod = h_mod.unwrap_or(PDU_HANDLE_UNDEF);

        trace!(func = FUNC, h_mod, "D-PDU API Call Args");

        let module_disconnect_fn =
            self.get_pdu_function::<PduModuleDisconnectFn>(FUNC.as_bytes())?;
        let result = module_disconnect_fn(h_mod);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_cancel_com_primitive(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        h_cop: PduCopHandle,
    ) -> ApiResult<()> {
        const FUNC: &'static str = "PDUCancelComPrimitive";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, h_cll, h_cop, "D-PDU API Call Args");

        let cancel_com_primitive_fn =
            self.get_pdu_function::<PduCancelComPrimitiveFn>(FUNC.as_bytes())?;
        let result = cancel_com_primitive_fn(h_mod, h_cll, h_cop);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_get_conflicting_resources(
        &self,
        resource_id: PduObjectId,
        modules: Vec<PduModuleData>,
    ) -> ApiResult<PduConflictingModules> {
        const FUNC: &'static str = "PDUGetConflictingResources";
        self.log_api_call(FUNC);

        trace!(func = FUNC, resource_id, "D-PDU API Call Args");

        let mut module_names: Vec<CString> = vec![];
        let mut module_infos: Vec<CString> = vec![];

        let module_items = modules
            .iter()
            .map(|m| {
                trace!(
                    func = FUNC,
                    h_mod = m.h_mod,
                    module_type_id = m.module_type_id,
                    "D-PDU API Call Args"
                );

                let module_name_idx = module_names.len();
                let module_info_idx = module_infos.len();

                module_names.push(
                    CString::new(m.vendor_module_name.clone().unwrap_or_else(String::new))
                        .expect("CString::new()"), // infallible
                );

                module_infos.push(
                    CString::new(m.vendor_additional_info.clone().unwrap_or_else(String::new))
                        .expect("CString::new()"), // infallible
                );

                ModuleData {
                    module_type_id: m.module_type_id,
                    h_mod: m.h_mod,
                    vendor_module_name: module_names[module_name_idx].as_ptr() as _,
                    vendor_additional_info: module_infos[module_info_idx].as_ptr() as _,
                    status: m.status,
                }
            })
            .collect::<Vec<_>>();

        let module_data = ModuleItem {
            item_type: PduIt::ModuleId,
            num_entries: modules.len() as _,
            p_module_data: module_items.as_ptr() as _,
        };

        let mut conflict_data_ptr = ptr::null_mut();

        trace!(
            func = FUNC,
            input_module_list_ptr = format!("{:#x}", &module_data as *const _ as usize),
            output_conflict_list_ptr = format!("{:#x}", &conflict_data_ptr as *const _ as usize),
            "D-PDU API Call Args"
        );

        let get_conflicting_resources_fn =
            self.get_pdu_function::<PduGetConflictingResourcesFn>(FUNC.as_bytes())?;
        let result = get_conflicting_resources_fn(
            resource_id,
            &module_data as *const _ as _,
            &mut conflict_data_ptr,
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        trace!(
            func = FUNC,
            item_ptr = format!("0x{:x}", conflict_data_ptr as usize),
            item_type = ?NonNull::new(conflict_data_ptr).map(|wptr| unsafe { (&*wptr.as_ptr()).item_type }),
            "D-PDU API Call Return"
        );

        if conflict_data_ptr.is_null() {
            error!(
                func = FUNC,
                "Item data pointer is null. Emulation of PduError::FctFailed..."
            );
            return Err(PduError::FctFailed)?;
        }

        let conflict_data = unsafe { &*conflict_data_ptr };

        if !matches!(conflict_data.item_type, PduIt::RscConflict) {
            error!(
                func = FUNC,
                "Invalid item type received: PduIt::{}. Emulation of PduError::FctFailed...",
                conflict_data.item_type.as_str(),
            );

            self.pdu_destroy_item(conflict_data_ptr as _)?;
            return Err(PduError::FctFailed)?;
        }

        let ptr = conflict_data.p_rsc_conflict_data;
        let len = conflict_data.num_entries as usize;

        let conflict_items = if ptr.is_null() || len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(ptr, len) }
        };

        let map = conflict_items
            .iter()
            .map(|i| {
                trace!(
                    func = FUNC,
                    conflicting_h_mod = i.h_mod,
                    conflicting_resource_id = i.resource_id,
                    "D-PDU API Call Return"
                );
                (i.h_mod, i.resource_id)
            })
            .collect();

        self.pdu_destroy_item(conflict_data_ptr as _)?;

        Ok(map)
    }

    pub fn pdu_get_resource_ids(
        &self,
        h_mod: Option<PduModuleHandle>,
        bus: &BusSource,
        protocol: &ProtocolSource,
        pins: &[TargetPin],
    ) -> ApiResult<PduModulesResourcesIds> {
        const FUNC: &'static str = "PDUGetResourceIds";
        self.log_api_call(FUNC);

        let h_mod = h_mod.unwrap_or(PDU_HANDLE_UNDEF);

        trace!(
            func = FUNC,
            h_mod,
            %bus,
            %protocol,
            "D-PDU API Call Args",
        );

        let bus_id = bus.resolve_bus_id(self)?;
        let protocol_id = protocol.resolve_protocol_id(self)?;
        let pin_data = target_pins_to_pin_data(self, FUNC, pins)?;

        let resource_data = RscData {
            bus_type_id: bus_id,
            protocol_id,
            num_pin_data: pin_data.len() as _,
            p_dlc_pin_data: pin_data.as_ptr() as _,
        };

        let mut rsc_data_ptr = ptr::null_mut();

        let get_resource_ids_fn = self.get_pdu_function::<PduGetResourceIdsFn>(FUNC.as_bytes())?;
        let result = get_resource_ids_fn(h_mod, &resource_data as *const _ as _, &mut rsc_data_ptr);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        trace!(
            func = FUNC,
            item_ptr = format!("0x{:x}", rsc_data_ptr as usize),
            item_type = ?NonNull::new(rsc_data_ptr).map(|wptr| unsafe { (&*wptr.as_ptr()).item_type }),
            "D-PDU API Call Return"
        );

        if rsc_data_ptr.is_null() {
            error!(
                func = FUNC,
                "Item data pointer is null. Emulation of PduError::FctFailed..."
            );
            return Err(PduError::FctFailed)?;
        }

        let rsc_data = unsafe { &*rsc_data_ptr };

        if !matches!(rsc_data.item_type, PduIt::RscConflict) {
            error!(
                func = FUNC,
                "Invalid item type received: PduIt::{}. Emulation of PduError::FctFailed...",
                rsc_data.item_type.as_str(),
            );

            self.pdu_destroy_item(rsc_data_ptr as _)?;
            return Err(PduError::FctFailed)?;
        }

        let mut map = PduModulesResourcesIds::with_capacity(rsc_data.num_modules as _);

        let rsc_items_ptr = rsc_data.p_id_item_data;
        let rsc_items_len = rsc_data.num_modules as usize;

        let rsc_items = if rsc_items_ptr.is_null() || rsc_items_len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(rsc_items_ptr, rsc_items_len) }
        };

        for rsc_item in rsc_items {
            let rsc_ids_ptr = rsc_item.p_resource_id_array;
            let rsc_ids_len = rsc_item.num_ids as usize;

            let resource_ids = if rsc_ids_ptr.is_null() || rsc_ids_len == 0 {
                &[]
            } else {
                unsafe { slice::from_raw_parts(rsc_ids_ptr, rsc_ids_len) }
            };

            trace!(
                func = FUNC,
                rsc_item_h_mod = rsc_item.h_mod,
                rsc_item_resource_ids = ?resource_ids,
                "D-PDU API Call Return"
            );

            map.insert(rsc_item.h_mod, resource_ids.to_vec());
        }

        Ok(map)
    }

    pub fn pdu_get_timestamp(&self, h_mod: PduModuleHandle) -> ApiResult<u32> {
        const FUNC: &'static str = "PDUGetTimestamp";
        self.log_api_call(FUNC);

        let mut timestamp = MaybeUninit::uninit();

        trace!(
            func = FUNC,
            h_mod,
            timestamp_ptr = format!("{:#x}", timestamp.as_ptr() as usize),
            "D-PDU API Call Args"
        );

        let get_timestamp_fn = self.get_pdu_function::<PduGetTimestampFn>(FUNC.as_bytes())?;
        let result = get_timestamp_fn(h_mod, timestamp.as_mut_ptr());

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        let timestamp = unsafe { timestamp.assume_init() };

        trace!(func = FUNC, timestamp, "D-PDU API Call Return");

        Ok(timestamp)
    }

    pub fn pdu_get_unique_resp_id_table(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
    ) -> ApiResult<PduComParamTable> {
        const FUNC: &'static str = "PDUGetUniqueRespIdTable";
        self.log_api_call(FUNC);

        let mut table_item_ptr = ptr::null_mut();

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            item_ptr = format!("{:#x}", &table_item_ptr as *const _ as usize),
            "D-PDU API Call Return"
        );

        let get_timestamp_fn =
            self.get_pdu_function::<PduGetUniqueRespIdTableFn>(FUNC.as_bytes())?;
        let result = get_timestamp_fn(h_mod, h_cll, &mut table_item_ptr);

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        trace!(
            func = FUNC,
            item_ptr = format!("0x{:x}", table_item_ptr as usize),
            item_type = ?NonNull::new(table_item_ptr).map(|wptr| unsafe { (&*wptr.as_ptr()).item_type }),
            "D-PDU API Call Return"
        );

        if table_item_ptr.is_null() {
            error!(
                func = FUNC,
                "Item data pointer is null. Emulation of PduError::FctFailed..."
            );
            return Err(PduError::FctFailed)?;
        }

        let table_item = unsafe { &*table_item_ptr };

        if !matches!(table_item.item_type, PduIt::RscConflict) {
            error!(
                func = FUNC,
                "Invalid item type received: PduIt::{}. Emulation of PduError::FctFailed...",
                table_item.item_type.as_str(),
            );

            self.pdu_destroy_item(table_item_ptr as _)?;
            return Err(PduError::FctFailed)?;
        }

        let table_ptr = table_item.p_unique_data;
        let table_len = table_item.num_entries as usize;

        let table = if table_ptr.is_null() || table_len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(table_ptr, table_len) }
        };

        let mut map = PduComParamTable::with_capacity(table.len());

        trace!(
            func = FUNC,
            table_len = table.len(),
            "D-PDU API Call Return"
        );

        for row in table {
            let unique_id = row.unique_resp_identifier;

            let com_params_ptr = row.p_params;
            let com_params_len = row.num_param_items as usize;

            let com_params = if com_params_ptr.is_null() || com_params_len == 0 {
                &[]
            } else {
                unsafe { slice::from_raw_parts(com_params_ptr, com_params_len) }
            };

            trace!(
                func = FUNC,
                table_item_unique_id = unique_id,
                table_item_cp_len = com_params.len(),
                "D-PDU API Call Return"
            );

            for cp in com_params {
                if !matches!(cp.item_type, PduIt::Param) {
                    error!(
                        func = FUNC,
                        "Invalid ComParam type received: PduIt::{}. Emulation of PduError::FctFailed...",
                        cp.item_type.as_str(),
                    );

                    self.pdu_destroy_item(table_item_ptr as _)?;
                    return Err(PduError::FctFailed)?;
                }

                trace!(
                    func = FUNC,
                    table_item_cp_id = cp.com_param_id,
                    table_item_cp_type = cp.com_param_data_type.as_str(),
                    table_item_cp_class = cp.com_param_class.as_str(),
                    "D-PDU API Call Return"
                );

                let variant: CpVariant = unsafe {
                    use ptr::read;

                    match cp.com_param_data_type {
                        PduPt::Unum8 => read::<u8>(cp.p_com_param_data as _).into(),
                        PduPt::Snum8 => read::<i8>(cp.p_com_param_data as _).into(),
                        PduPt::Unum16 => read::<u16>(cp.p_com_param_data as _).into(),
                        PduPt::Snum16 => read::<i16>(cp.p_com_param_data as _).into(),
                        PduPt::Unum32 => read::<u32>(cp.p_com_param_data as _).into(),
                        PduPt::Snum32 => read::<i32>(cp.p_com_param_data as _).into(),
                        PduPt::ByteField => {
                            let data = read::<ParamByteFieldData>(cp.p_com_param_data as _);
                            let ptr = data.p_data_array;
                            let len = data.param_act_len as usize;
                            let bytes = if ptr.is_null() || len == 0 {
                                &[]
                            } else {
                                slice::from_raw_parts(ptr, len)
                            };
                            (bytes.to_vec(), data.param_max_len as usize).into()
                        }
                        PduPt::LongField => {
                            let data = read::<ParamLongFieldData>(cp.p_com_param_data as _);
                            let ptr = data.p_data_array;
                            let len = data.param_act_len as usize;
                            let nums = if ptr.is_null() || len == 0 {
                                &[]
                            } else {
                                slice::from_raw_parts(ptr, len)
                            };
                            (nums.to_vec(), data.param_max_len as usize).into()
                        }
                        PduPt::StructField => {
                            let data = read::<ParamStructFieldData>(cp.p_com_param_data as _);
                            let ptr = data.p_struct_array;
                            let len = data.param_act_entries as usize;
                            match data.com_param_struct_type {
                                PduCpst::AccessTiming => {
                                    let structs: &[ParamStructAccessTiming] = if ptr.is_null() || len == 0 {
                                        &[]
                                    } else {
                                        slice::from_raw_parts(ptr as _, len)
                                    };
                                    (structs.to_vec(), data.param_max_entries as usize).into()
                                }
                                PduCpst::SessionTiming => {
                                    let structs: &[ParamStructSessionTiming] = if ptr.is_null() || len == 0 {
                                        &[]
                                    } else {
                                        slice::from_raw_parts(ptr as _, len)
                                    };
                                    (structs.to_vec(), data.param_max_entries as usize).into()
                                }
                            }
                        }
                    }
                };

                let com_param = PduComParam::from_id(cp.com_param_id, cp.com_param_class, variant);

                com_param.try_init_short_name(self);

                map.add(unique_id, com_param);
            }
        }

        self.pdu_destroy_item(table_item_ptr as _)?;

        Ok(map)
    }
}

fn target_pins_to_pin_data(
    api: &PduApi,
    func_name: &str,
    pins: &[TargetPin],
) -> ApiResult<Vec<PinData>> {
    let mut vec = Vec::with_capacity(pins.len());

    for pin in pins.iter() {
        trace!(
            func = func_name,
            pin_num = pin.num_on_vci,
            pin_type = %pin.pin_type,
            "D-PDU API Call Args"
        );

        let pin_id = match &pin.pin_type {
            PinSource::Id(id) => id.clone(),
            PinSource::Name(name) => api.pdu_get_object_id(PduObjt::PinType, name)?,
        };

        vec.push(PinData {
            dlc_pin_number: pin.num_on_vci,
            dlc_pin_type_id: pin_id,
        });
    }

    Ok(vec)
}
