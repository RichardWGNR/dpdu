use std::ffi::{CString};
use std::{ptr, slice};
use std::cell::{OnceCell};
use std::collections::{HashMap, HashSet};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::pin::{pin, Pin};
use std::ptr::NonNull;
use std::sync::{Arc, Weak};
use dpdu_api_types::{EcuUniqueRespData, ErrorData, EventCallbackFn, EventItem, InfoData, ParamByteFieldData, ParamItem, ParamLongFieldData, ParamStructFieldData, PduConstructFn, PduDestroyItemFn, PduDestructFn, PduError, PduGetComParamFn, PduGetEventItemFn, PduGetObjectIdFn, PduGetVersionFn, PduIt, PduItem, PduObjt, PduPc, PduPt, PduRegisterCallbackFn, PduSetComParamFn, PduSetUniqueRespIdTableFn, PduStatus, ResultData, UniqueRespIdTableItem, VersionData, PDU_HANDLE_UNDEF, PDU_ID_UNDEF};
use rand::random;
use tracing::{debug, error, trace, warn};
use crate::types::{PduCllHandle, PduLibraryPath, PduModuleHandle, PduObjectId, PduOptions, PduUniqueId};
use crate::types::pdu_com_param::{FieldComParam, PduComParam, PduCpVariant, StructComParam};
use crate::types::pdu_com_param_table::PduComParamTable;
use crate::types::pdu_event::{PduErrorEvent, PduEvent, PduEventData, PduInfoEvent, PduResultEvent, PduStatusEvent};
use crate::types::pdu_event_callback::PduEventCallbackTarget;
use crate::types::pdu_object::PduObjectIdSource;
use crate::types::pdu_version::PduVersionData;
use crate::utils::c_str;
use crate::utils::module_description::PduModuleDescription;
use crate::utils::root_file::Mvci;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("FFI error: {0}")]
    FfiError(#[from] libloading::Error),

    #[error("Communication error: {0}")]
    CommError(#[from] PduError),
}

#[derive(Debug)]
pub struct Api {
    me: Weak<Api>,

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
        mvci: Option<Mvci>
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

    fn log_api_call(&self, func: &str) {
        debug!(func, "D-PDU API Call");
    }

    fn log_failed_api_call(&self, func: &str, result: PduError) {
        error!(
            func,
            result_str = result.as_ref(),
            result_int = format!("{:#X}", result as usize),
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
                error!(function, "Unable to take a pointer to the D-PDU API function: {err}");
                Err(err)?
            },
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
            self.pdu_unique_id as *const PduUniqueId as _
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

        trace!(func = FUNC, p_item = format!("0x{:x}", item_ptr as usize), "D-PDU API Call Args");

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

    pub fn pdu_get_event_item(&self, h_mod: PduModuleHandle, h_cll: PduCllHandle) -> Result<Option<PduEvent>> {
        const FUNC: &'static str = "PDUGetEventItem";
        self.log_api_call(FUNC);

        trace!(func = FUNC, h_mod, h_cll, "D-PDU API Call Args");

        let mut item_ptr: *mut EventItem = ptr::null_mut();

        let get_event_item_fn = self.get_pdu_function::<PduGetEventItemFn>(FUNC.as_bytes())?;
        let result = get_event_item_fn(h_mod, h_cll, &mut item_ptr);

        match result {
            PduError::StatusNoError | PduError::EventQueueEmpty => {},
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

        assert_eq!(item_ptr.is_null(), false, "item_ptr is null");

        let item = unsafe { &*item_ptr };

        fn ensure_item_data_ptr_is_not_null(item: &EventItem) {
            assert_eq!(item.p_data.is_null(), false, "item.p_data ptr is null");
        }

        let data: PduEventData = match item.item_type {
            PduIt::Status => {
                ensure_item_data_ptr_is_not_null(item);
                PduStatusEvent(unsafe { *(item.p_data as *const PduStatus) }).into()
            },
            PduIt::Result => {
                ensure_item_data_ptr_is_not_null(item);
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
                }.into()
            },
            PduIt::Error => {
                ensure_item_data_ptr_is_not_null(item);
                let data = unsafe { &*(item.p_data as *const ErrorData) };
                PduErrorEvent {
                    code: data.error_code_id,
                    extra_code: data.extra_error_info_id,
                }.into()
            },
            PduIt::Info => {
                ensure_item_data_ptr_is_not_null(item);
                let data = unsafe { &*(item.p_data as *const InfoData) };
                PduInfoEvent {
                    code: data.info_code,
                    extra_code: data.extra_info_data,
                }.into()
            },
            typ => {
                unreachable!("Unexpected PduEventItem type: {}", typ.as_ref());
            }
        };

        self.pdu_destroy_item(item_ptr as _)?;

        Ok(Some(PduEvent {
            h_mod: (h_mod != PDU_HANDLE_UNDEF).then(|| h_mod),
            h_cll: (h_cll != PDU_HANDLE_UNDEF).then(|| h_cll),
            h_cop: (item.h_cop != PDU_HANDLE_UNDEF).then(|| item.h_cop),
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

    pub fn pdu_get_object_id(
        &self,
        object: PduObjt,
        short_name: &str,
    ) -> Result<PduObjectId> {
        const FUNC: &'static str = "PDUGetObjectId";
        self.log_api_call(FUNC);

        trace!(func = FUNC, object = object.as_ref(), short_name, "D-PDU API Call Args");

        if let Some(desc) = &self.module_description {
            // First, we will try to obtain the required object ID from the module description
            // file supplied with the D-PDU API driver in order to reduce
            // the number of D-PDU API calls.
            let object_id = match object {
                PduObjt::IoCtrl => desc.io_controls
                    .get_by_short_name(short_name)
                    .map(|v| v.id),
                PduObjt::Resource => desc.resources
                    .get_by_short_name(short_name)
                    .map(|v| v.id),
                PduObjt::Protocol => desc.protocols
                    .get_by_short_name(short_name)
                    .map(|v| v.id),
                PduObjt::BusType => desc.bus_types
                    .get_by_short_name(short_name)
                    .map(|v| v.id),
                PduObjt::PinType => desc.pin_types
                    .get_by_short_name(short_name)
                    .map(|v| v.id),
                PduObjt::ComParam => desc.com_params
                    .get_by_short_name(short_name)
                    .map(|v| v.id),
            };

            if let Some(id) = object_id {
                return Ok(id);
            }
        }

        let short_name = CString::new(short_name).expect("CString::new() failed");
        let mut object_id: MaybeUninit<u32> = MaybeUninit::uninit();

        let get_object_id_fn = self.get_pdu_function::<PduGetObjectIdFn>(FUNC.as_bytes())?;
        let result = get_object_id_fn(
            object,
            short_name.as_ptr() as _,
            object_id.as_mut_ptr()
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        // SAFETY:
        // PDUGetObjectId guarantees that `object_id` is initialized on success.
        Ok(unsafe { object_id.assume_init() })
    }

    pub fn pdu_get_com_param(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        object_id: PduObjectIdSource
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
                },
                _ => {
                    self.log_failed_api_call(FUNC, result);
                    Err(result)?
                }
            }
        }

        assert_eq!(item_ptr.is_null(), false, "item_ptr is null");

        let cp = unsafe {
            use ptr::read as read;

            let item = &*item_ptr;
            let data_ptr = item.p_com_param_data;
            let short_name = OnceCell::new();

            match &object_id {
                PduObjectIdSource::ShortName(v) => {
                    let _ = short_name.set(v.clone());
                },
                _ => {
                    let sn_opt = self.module_description
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
                        FieldComParam::<u8, ParamByteFieldData>::new_byte_field(
                            slice::from_raw_parts(
                                data.p_data_array,
                                data.param_act_len as _,
                            ).to_vec(),
                            Some(data.param_max_len as _),
                        )
                    }),
                    PduPt::StructField => PduCpVariant::StructField({
                        let data = &*(data_ptr as *const ParamStructFieldData);
                        FieldComParam::<StructComParam, ParamStructFieldData>::new_struct_field(
                            data.com_param_struct_type,
                            slice::from_raw_parts(
                                data.p_struct_array as *mut StructComParam,
                                data.param_act_entries as _,
                            ).to_vec(),
                            Some(data.param_max_entries as _),
                        )
                    }),
                    PduPt::LongField => PduCpVariant::LongField({
                        let data = &*(data_ptr as *const ParamLongFieldData);
                        FieldComParam::<u32, ParamLongFieldData>::new_long_field(
                            slice::from_raw_parts(
                                data.p_data_array,
                                data.param_act_len as _,
                            ).to_vec(),
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
                },
                _ => {
                    self.log_failed_api_call(FUNC, result);
                    Err(result)?
                }
            }
        }

        Ok(())
    }

    pub fn pdu_set_unique_resp_id_table(
        &self,
        h_mod: PduModuleHandle,
        h_cll: PduCllHandle,
        table: &PduComParamTable
    ) -> Result<()> {
        const FUNC: &'static str = "PDUSetUniqueRespIdTable";
        self.log_api_call(FUNC);

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            "D-PDU API Call Args"
        );

        type EphemeralGroupKey = usize;

        let mut temp_com_param_groups: HashMap<EphemeralGroupKey, Vec<ParamItem>> = HashMap::with_capacity(table.len());
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

            let temp_com_param_group = temp_com_param_groups
                .get(&key)
                .expect("inserted above");

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

        let set_unique_resp_id_table_fn = self.get_pdu_function::<PduSetUniqueRespIdTableFn>(FUNC.as_bytes())?;
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
        callback: Option<EventCallbackFn>
    ) -> Result<()> {
        const FUNC: &'static str = "PDUSetUniqueRespIdTable";
        self.log_api_call(FUNC);

        trace!(
            func = FUNC,
            target = target.as_ref(),
            "D-PDU API Call Args"
        );

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
            },
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
            },
            PduEventCallbackTarget::System => (PDU_HANDLE_UNDEF, PDU_HANDLE_UNDEF)
        };

        trace!(
            func = FUNC,
            h_mod,
            h_cll,
            "D-PDU API Call Args"
        );

        let register_event_callback_fn =
            self.get_pdu_function::<PduRegisterCallbackFn>(FUNC.as_bytes())?;

        let result = register_event_callback_fn(
            h_mod,
            h_cll,
            unsafe { std::mem::transmute(callback) },
        );

        if !result.is_success() {
            self.log_failed_api_call(FUNC, result);
            return Err(result)?;
        }

        Ok(())
    }
}