use crate::types::pdu_com_logical_link::{
    CllBusType, CllCreateFlags, CllCreateType, CllPinType, CllProtocolType, PduComLogicalLink,
};
use crate::types::pdu_com_param::{
    ByteFieldComParam, LongFieldComParam, PduComParam, PduCpVariant, StructComParam,
    StructFieldComParam,
};
use crate::types::pdu_com_param_table::PduComParamTable;
use crate::types::pdu_com_primivite::PduComPrimiviteParams;
use crate::types::pdu_error::{PduErrorData, PduLastErrorTarget};
use crate::types::pdu_event::{
    PduErrorEvent, PduEvent, PduEventData, PduEventTarget, PduInfoEvent, PduResultEvent,
    PduStatusEvent,
};
use crate::types::pdu_event_callback::PduEventCallbackTarget;
use crate::types::pdu_io_ctl::{IoCtlByteArray, PduIoCtlCommand, PduIoCtlData, PduIoCtlTarget};
use crate::types::pdu_lock_resource::PduLockResourceMask;
use crate::types::pdu_module::{PduModule, PduModuleList};
use crate::types::pdu_object::PduObjectIdSource;
use crate::types::pdu_resource::{PduResource, ResourceStatus};
use crate::types::pdu_status::{PduStatusData, PduStatusTarget};
use crate::types::pdu_version::PduVersionData;
use crate::types::{
    PduCllHandle, PduCopHandle, PduLibraryPath, PduModuleHandle, PduObjectId, PduOptions,
    PduUniqueId,
};
use crate::utils::c_str;
use crate::utils::module_description::{PduModuleDescription, PduModuleDescriptionError};
use crate::utils::root_file::Mvci;
use dpdu_api_types::{
    CopCtrlData, EcuUniqueRespData, ErrorData, EventCallbackFn, EventItem, ExpRespData,
    FlagData, InfoData, IoByteArrayData, IoEventQueuePropertyData, IoFilterData, IoProgVoltageData, PDU_HANDLE_UNDEF, PDU_ID_UNDEF, ParamByteFieldData, ParamItem, ParamLongFieldData,
    ParamStructFieldData, PduConnectFn, PduConstructFn, PduCopt, PduCreateComLogicalLinkFn, PduDestroyComLogicalLinkFn, PduDestroyItemFn, PduDestructFn, PduDisconnectFn,
    PduError, PduErrorEvt, PduGetComParamFn, PduGetEventItemFn, PduGetLastErrorFn,
    PduGetModuleIdsFn, PduGetObjectIdFn, PduGetResourceStatusFn, PduGetStatusFn, PduGetVersionFn,
    PduIoctlFn, PduIt, PduItem, PduLockResourceFn, PduObjt, PduPc, PduPt, PduRegisterCallbackFn,
    PduSetComParamFn, PduSetUniqueRespIdTableFn, PduStartComPrimitiveFn, PduStatus,
    PduUnlockResourceFn, PinData, ResultData, RscData, RscStatusData, RscStatusItem,
    UniqueRespIdTableItem, VersionData,
};
use rand::random;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::ffi::{CString, c_void};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::{Arc, Weak};
use std::{ptr, slice};
use tracing::{debug, error, trace, warn};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FFI error: {0}")]
    FfiError(#[from] libloading::Error),

    #[error("Communication error: {0}")]
    CommError(#[from] PduError),

    #[error("Module description error: {0}")]
    MdfError(#[from] PduModuleDescriptionError),
}

#[derive(Debug)]
pub struct Api {
    pub(crate) me: Weak<Api>,

    pdu_options: PduOptions,

    pdu_unique_id: PduUniqueId,

    library: libloading::Library,

    library_file: Option<PduLibraryPath>,

    pub(crate) module_description: Option<PduModuleDescription>,

    mvci: Option<Mvci>,
}

impl Api {
    pub fn new(
        options: PduOptions,
        library: libloading::Library,
        library_file: Option<PduLibraryPath>,
        module_description: Option<PduModuleDescription>,
        mvci: Option<Mvci>,
    ) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            me: me.clone(),
            pdu_options: options,
            pdu_unique_id: random(),
            library,
            library_file,
            module_description,
            mvci,
        })
    }

    pub fn from_mvci(mvci: &Mvci, options: PduOptions) -> Result<Arc<Self>> {
        let library= unsafe { libloading::Library::new(&mvci.library_file)? };
        let mdf = mvci.module_description_file
            .as_ref()
            .map(|v| PduModuleDescription::parse_from_xml_file(v))
            .transpose()?;

        Ok(Api::new(
            options,
            library,
            Some(mvci.library_file.clone()),
            mdf,
            Some(mvci.clone())
        ))
    }

    pub fn from_library_path(
        library_file: impl Into<PduLibraryPath>,
        options: PduOptions,
        module_description: Option<PduModuleDescription>,
    ) -> Result<Arc<Self>> {
        let library_file = library_file.into();
        let library = unsafe { libloading::Library::new(&library_file)? };

        Ok(Api::new(
            options,
            library,
            Some(library_file),
            module_description,
            None
        ))
    }
    
    pub fn from_library(
        library: libloading::Library,
        options: PduOptions,
        module_description: Option<PduModuleDescription>,
    ) -> Result<Arc<Self>> {
        Ok(Api::new(
            options,
            library,
            None,
            module_description,
            None
        ))
    }

    fn log_api_call(&self, func: &str) {
        debug!(func, "D-PDU API Call");
    }

    fn log_failed_api_call(&self, func: &str, result: PduError) {
        error!(
            func,
            result_str = result.as_ref(),
            result_int = format!("0x{:#x}", result as usize),
            "D-PDU API Call failed"
        );
    }

    fn get_pdu_function<F>(&self, name: &[u8]) -> Result<libloading::Symbol<'_, F>> {
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

    pub fn pdu_construct(&self) -> Result<()> {
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
            self.pdu_unique_id as *const PduUniqueId as _,
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }

    pub fn pdu_destruct(&self) -> Result<()> {
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

    pub fn pdu_destroy_item(&self, item_ptr: *mut PduItem) -> Result<()> {
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

    pub fn pdu_get_event_item(&self, target: PduEventTarget) -> Result<Option<PduEvent>> {
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
            Err(PduError::FctFailed)?;
        }

        let item = unsafe { &*item_ptr };

        if item.p_data.is_null() {
            error!(
                func = FUNC,
                "Item data pointer is null. Emulation of PduError::FctFailed..."
            );
            Err(PduError::FctFailed)?;
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
                        let len = extra_info.p_header_bytes;
                        extra_header
                            .set(unsafe { slice::from_raw_parts(ptr, len as _) }.to_vec())
                            .unwrap();
                    }
                    if !extra_info.p_footer_bytes.is_null() {
                        let ptr = extra_info.p_footer_bytes;
                        let len = extra_info.num_footer_bytes;
                        extra_footer
                            .set(unsafe { slice::from_raw_parts(ptr, len as _) }.to_vec())
                            .unwrap();
                    }
                }

                PduResultEvent {
                    rx_flags: unsafe {
                        let ptr = data.rx_flag.p_flag_data;
                        let len = data.rx_flag.num_flag_bytes;
                        slice::from_raw_parts(ptr, len as _).to_vec().into()
                    },
                    unique_resp_identifier: data.unique_resp_identifier,
                    acceptance_id: data.acceptance_id,
                    timestamp_flags: unsafe {
                        let ptr = data.timestamp_flags.p_flag_data;
                        let len = data.timestamp_flags.num_flag_bytes;
                        slice::from_raw_parts(ptr, len as _).to_vec().into()
                    },
                    tx_msg_done_timestamp: data.tx_msg_done_timestamp,
                    start_msg_timestamp: data.start_msg_timestamp,
                    data: unsafe {
                        let ptr = data.p_data_bytes;
                        let len = data.num_data_bytes;
                        slice::from_raw_parts(ptr, len as _).to_vec()
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
                    typ.as_ref()
                );
                return Err(PduError::FctFailed)?;
            }
        };

        self.pdu_destroy_item(item_ptr as _)?;

        Ok(Some(PduEvent {
            target,
            timestamp: item.timestamp,
            data,
        }))
    }

    pub fn pdu_get_version(&self, h_mod: PduModuleHandle) -> Result<PduVersionData> {
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

    pub fn pdu_get_object_id(&self, object: PduObjt, short_name: &str) -> Result<PduObjectId> {
        const FUNC: &'static str = "PDUGetObjectId";
        self.log_api_call(FUNC);

        trace!(
            func = FUNC,
            object = object.as_ref(),
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
    ) -> Result<PduComParam> {
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
            Err(PduError::FctFailed)?;
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
                    PduPt::Unum8 => PduCpVariant::Unum8(read(data_ptr as _)),
                    PduPt::Snum8 => PduCpVariant::Snum8(read(data_ptr as _)),
                    PduPt::Unum16 => PduCpVariant::Unum16(read(data_ptr as _)),
                    PduPt::Snum16 => PduCpVariant::Snum16(read(data_ptr as _)),
                    PduPt::Unum32 => PduCpVariant::Unum32(read(data_ptr as _)),
                    PduPt::Snum32 => PduCpVariant::Snum32(read(data_ptr as _)),
                    PduPt::ByteField => PduCpVariant::ByteField({
                        let data = &*(data_ptr as *const ParamByteFieldData);
                        ByteFieldComParam::new(
                            slice::from_raw_parts(data.p_data_array, data.param_act_len as _)
                                .to_vec(),
                            Some(data.param_max_len as _),
                        )
                    }),
                    PduPt::StructField => PduCpVariant::StructField({
                        let data = &*(data_ptr as *const ParamStructFieldData);
                        StructFieldComParam::new(
                            data.com_param_struct_type,
                            slice::from_raw_parts(
                                data.p_struct_array as *mut StructComParam,
                                data.param_act_entries as _,
                            )
                            .to_vec(),
                            Some(data.param_max_entries as _),
                        )
                    }),
                    PduPt::LongField => PduCpVariant::LongField({
                        let data = &*(data_ptr as *const ParamLongFieldData);
                        LongFieldComParam::new(
                            slice::from_raw_parts(data.p_data_array, data.param_act_len as _)
                                .to_vec(),
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
    ) -> Result<()> {
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
    ) -> Result<()> {
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
                        class = cp.class.as_ref(),
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
                    com_param_ptr = format!("0x{:#X}", &item as *const _ as usize),
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
            table_ptr = format!("0x{:#X}", &table as *const _ as usize),
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
        target: PduEventCallbackTarget,
        callback: Option<EventCallbackFn>,
    ) -> Result<()> {
        const FUNC: &'static str = "PDURegisterEventCallback";
        self.log_api_call(FUNC);

        trace!(func = FUNC, target = target.as_ref(), "D-PDU API Call Args");

        let (h_mod, h_cll) = match target {
            PduEventCallbackTarget::Module(h_mod) => {
                if h_mod == PDU_HANDLE_UNDEF {
                    let result = PduError::InvalidHandle;
                    error!(
                        func = FUNC,
                        "Module handle of the PduEventCallbackTarget cannot be PDU_HANDLE_UNDEF"
                    );
                    self.log_failed_api_call(FUNC, result);
                    return Err(result)?;
                }

                (h_mod, PDU_HANDLE_UNDEF)
            }
            PduEventCallbackTarget::ComLogicalLink(h_mod, h_cll) => {
                if h_mod == PDU_HANDLE_UNDEF {
                    let result = PduError::InvalidHandle;
                    error!(
                        func = FUNC,
                        "Module handle of the PduEventCallbackTarget cannot be PDU_HANDLE_UNDEF"
                    );
                    self.log_failed_api_call(FUNC, result);
                    return Err(result)?;
                } else if h_cll == PDU_HANDLE_UNDEF {
                    let result = PduError::InvalidHandle;
                    error!(
                        func = FUNC,
                        "ComLogicalLink handle of the PduEventCallbackTarget cannot be PDU_HANDLE_UNDEF"
                    );
                    self.log_failed_api_call(FUNC, result);
                    return Err(result)?;
                }

                (h_mod, h_cll)
            }
            PduEventCallbackTarget::System => (PDU_HANDLE_UNDEF, PDU_HANDLE_UNDEF),
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

    pub fn pdu_start_com_primivite(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        cop_type: PduCopt,
        data: &[u8],
        params: Option<&PduComPrimiviteParams>,
    ) -> Result<PduCopHandle> {
        const FUNC: &'static str = "PDUStartComPrimitive";
        self.log_api_call(FUNC);

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            cop_type = cop_type.as_ref(),
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
                    tag = "<nullptr>",
                    cop_handle_ptr = format!("0x{:#x}", cop_handle.as_ptr() as usize),
                    "D-PDU API Call Args"
                );

                start_com_primivite_fn(
                    h_mod,
                    h_cll,
                    cop_type,
                    0,               // data len
                    ptr::null_mut(), // data ptr
                    ptr::null_mut(), // cop ctrl data
                    ptr::null_mut(), // tag
                    cop_handle.as_mut_ptr(),
                )
            }
            _ => {
                let params = params.expect(&format!(
                    "when PduCopt = {}, PduComPrimitiveParams is required",
                    cop_type.as_ref()
                ));

                let flags = params.tx_flag.get_pdu_flag_data();

                trace!(
                    func = FUNC,

                    cop_delay_ms = params.time,
                    send_cycles = params.send_cycles.to_i32(),
                    receive_cycles = params.receive_cycles.to_i32(),
                    buffer = params.temp_param_update.as_ref(),

                    flags_ptr = format!("0x{:#x}", flags.as_ptr() as usize),
                    flags = ?flags,

                    expected_responses_len = params.expected_responses.len(),

                    "D-PDU API Call Args"
                );

                let expected_responses = params.expected_responses
                    .iter()
                    .map(|v| {
                        trace!(
                            func = FUNC,

                            response_type = v.response_type.as_ref(),
                            acceptance_id = v.acceptance_id,

                            mask_data_ptr = format!("0x{:#x}", v.mask_data.get_mask().as_ptr() as usize),
                            mask_data_len = v.mask_data.mask.len(),
                            mask_data = ?v.mask_data.mask,

                            mask_pattern_ptr = format!("0x{:#x}", v.mask_data.get_pattern().as_ptr() as usize),
                            mask_pattern_len = v.mask_data.pattern.len(),
                            mask_pattern = ?v.mask_data.pattern,

                            unique_response_ids_ptr = format!("0x{:#x}", v.unique_response_ids.as_ptr() as usize),
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
                    data_ptr = format!("0x{:#x}", data.as_ptr() as usize),
                    cop_ctrl_data_ptr = format!("0x{:#x}", &cop_ctrl_data as *const _ as usize),
                    tag = "<nullptr>",
                    cop_handle_ptr = format!("0x{:#x}", cop_handle.as_ptr() as usize),
                    "D-PDU API Call Args"
                );

                start_com_primivite_fn(
                    h_mod,
                    h_cll,
                    cop_type,
                    data.len() as _,
                    data.as_ptr() as _,
                    &cop_ctrl_data as *const _ as _,
                    ptr::null_mut(), // tag
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

        Ok(cop_handle)
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
        target: PduIoCtlTarget,
        command: PduIoCtlCommand,
        data: Option<&PduIoCtlData>,
    ) -> Result<Option<PduIoCtlData>> {
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
                data_type = data.as_ref(),
                data_u32 = v,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::ProgVoltage(v) => trace!(
                func = FUNC,
                data_type = data.as_ref(),
                data_prog_voltage_mv = v.prog_voltage_mv,
                data_pin_on_dlc = v.pin_on_dlc,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::ByteArray(v) => trace!(
                func = FUNC,
                data_type = data.as_ref(),
                data_len = v.len(),
                data_value = ?v,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::Filter(v) => trace!(
                func = FUNC,
                data_type = data.as_ref(),
                data_filter_type = v.filter_type.as_ref(),
                data_filter_number = v.filter_number,
                data_filter_compare_size = v.filter_compare_size,
                data_filter_mask_msg = ?v.filter_mask_msg,
                data_filter_pattern_msg = ?v.filter_pattern_msg,
                "D-PDU API Call Args"
            ),
            PduIoCtlData::EventQueueProperty(v) => trace!(
                func = FUNC,
                data_type = data.as_ref(),
                data_queue_size = v.queue_size,
                data_queue_mode = v.queue_mode.as_ref(),
                "D-PDU API Call Args"
            ),
        });

        let object_id = match command {
            PduIoCtlCommand::Id(v) => v,
            PduIoCtlCommand::Name(v) => self.pdu_get_object_id(PduObjt::IoCtrl, &v)?,
        };

        let input_data_ptr: *const c_void = data
            .as_ref()
            .map(|v| v.to_pdu_data_item().p_data as _)
            .unwrap_or(ptr::null());

        let mut output_data_ptr = ptr::null_mut();

        trace!(
            func = FUNC,
            input_data_ptr = format!("0x{:#x}", input_data_ptr as usize),
            output_data_ptr = format!("0x{:#x}", &output_data_ptr as *const _ as usize),
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
            let io_ctl_data: Result<Option<PduIoCtlData>> = unsafe {
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
                                data_type = PduIt::IoByteArray.as_ref(),
                                "Byte array pointer is null. Emulation of PduError::FctFailed..."
                            );
                            Err(PduError::FctFailed.into())
                        } else {
                            let len = byte_array.data_size as _;
                            Ok(Some(
                                IoCtlByteArray(Vec::from_raw_parts(byte_array.p_data, len, len))
                                    .into(),
                            ))
                        }
                    }
                    PduIt::IoFilter => Ok(Some(data.p_data.cast::<IoFilterData>().read().into())),
                    PduIt::IoEventQueueProperty => Ok(Some(
                        data.p_data.cast::<IoEventQueuePropertyData>().read().into(),
                    )),
                    v => {
                        error!(
                            func = FUNC,
                            data_type = v.as_ref(),
                            "Unexpected output data type. Emulation of PduError::FctFailed..."
                        );
                        Err(PduError::FctFailed.into())
                    }
                }
            };

            self.pdu_destroy_item(output_data_ptr as _)?;

            io_ctl_data
        } else {
            Ok(None)
        }
    }

    pub fn pdu_get_module_ids(&self) -> Result<PduModuleList> {
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
            Err(PduError::FctFailed)?;
        }

        let module_list_item = unsafe { &*module_list_item_ptr };

        let ptr = module_list_item.p_module_data;
        let len = module_list_item.num_entries as _;

        trace!(
            func = FUNC,
            modules_ptr = format!("0x{:#x}", ptr as usize),
            modules_len = len,
            "D-PDU API Call Return"
        );

        let module_list = unsafe { Vec::from_raw_parts(ptr, len, len) }
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

                PduModule {
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

    pub fn pdu_get_status(&self, target: PduStatusTarget) -> Result<PduStatusData> {
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
                    "Received out-of-bounds PduStatus value: 0x{:#x}. Emulation of PduError::FctFailed...",
                    status_code,
                );
                return Err(PduError::FctFailed)?;
            }
        };

        Ok(PduStatusData {
            target,
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
    ) -> Result<PduComLogicalLink> {
        const FUNC: &'static str = "PDUCreateComLogicalLink";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, "D-PDU API Call Args");

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
                    ptr::null_mut(),
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

                let bus_type_id = match bus {
                    CllBusType::Id(id) => id.clone(),
                    CllBusType::Name(name) => self.pdu_get_object_id(PduObjt::BusType, name)?,
                };

                let protocol_id = match protocol {
                    CllProtocolType::Id(id) => id.clone(),
                    CllProtocolType::Name(name) => {
                        self.pdu_get_object_id(PduObjt::Protocol, name)?
                    }
                };

                let pin_data = {
                    let mut vec = Vec::with_capacity(pins.len());

                    for pin in pins.iter() {
                        trace!(
                            func = FUNC,
                            pin_num = pin.num_on_vci,
                            pin_type = %pin.pin_type,
                            "D-PDU API Call Args"
                        );

                        let pin_id = match &pin.pin_type {
                            CllPinType::Id(id) => id.clone(),
                            CllPinType::Name(name) => {
                                self.pdu_get_object_id(PduObjt::PinType, name)?
                            }
                        };

                        vec.push(PinData {
                            dlc_pin_number: pin.num_on_vci,
                            dlc_pin_type_id: pin_id,
                        });
                    }

                    vec
                };

                let rsc_data = RscData {
                    bus_type_id,
                    protocol_id,
                    num_pin_data: pin_data.len() as _,
                    p_dlc_pin_data: pin_data.as_ptr() as _,
                };

                trace!(
                    func = FUNC,
                    rsc_data_ptr = format!("0x{:#x}", &rsc_data as *const _ as usize),
                    bus_type_id,
                    protocol_id,
                    pin_len = pin_data.len(),
                    "D-PDU API Call Args"
                );

                create_com_logical_link_fn(
                    h_mod,
                    &rsc_data as *const RscData as _,
                    PDU_ID_UNDEF,
                    ptr::null_mut(),
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

        Ok(PduComLogicalLink {
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
    ) -> Result<()> {
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

    pub fn pdu_get_last_error(&self, target: PduLastErrorTarget) -> Result<PduErrorData> {
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
            error_code_ptr = format!("0x{:#x}", error_code.as_ptr() as usize),
            h_cop_ptr = format!("0x{:#x}", h_cop.as_ptr() as usize),
            timestamp_ptr = format!("0x{:#x}", timestamp.as_ptr() as usize),
            extra_info_code_ptr = format!("0x{:#x}", extra_info_code.as_ptr() as usize),
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
                    "Received out-of-bounds PduErrorEvt value: 0x{:#x}. Emulation of PduError::FctFailed...",
                    error_code,
                );
                return Err(PduError::FctFailed)?;
            }
        };

        Ok(PduErrorData {
            target,
            error_event,
            h_cop: (h_cop != PDU_HANDLE_UNDEF).then(|| h_cop),
            timestamp,
            extra_info_code: (extra_info_code != PDU_ID_UNDEF).then(|| extra_info_code),
        })
    }

    pub fn pdu_get_resource_status(
        &self,
        resources: Vec<PduResource>,
    ) -> Result<HashMap<PduResource, ResourceStatus>> {
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
            item_ptr = format!("0x{:#x}", &item as *const _ as usize),
            item_len = resources.len(),
            resources_ptr = format!("0x{:#x}", raw_resources.as_ptr() as usize),
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

    pub fn pdu_connect(&self, h_mod: PduModuleHandle, h_cll: PduCllHandle) -> Result<()> {
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

    pub fn pdu_disconnect(&self, h_mod: PduModuleHandle, h_cll: PduCllHandle) -> Result<()> {
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
    ) -> Result<()> {
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
    ) -> Result<()> {
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
}
