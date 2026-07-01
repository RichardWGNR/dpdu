use std::ffi::{c_void, CString};
use std::{ptr, slice};
use std::cell::{Cell, OnceCell};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::{Arc, Weak};
use dpdu_api_types::{ErrorData, EventItem, InfoData, PduConstructFn, PduDestroyItemFn, PduDestructFn, PduError, PduGetEventItemFn, PduGetObjectIdFn, PduGetVersionFn, PduIt, PduItem, PduObjt, PduStatus, ResultData, VersionData, PDU_HANDLE_UNDEF};
use rand::random;
use tracing::{debug, error, trace};
use crate::types::{PduCllHandle, PduLibraryPath, PduModuleHandle, PduOptions, PduUniqueId};
use crate::types::pdu_event::{PduErrorEvent, PduEvent, PduEventData, PduInfoEvent, PduResultEvent, PduResultEventRxFlags, PduStatusEvent};
use crate::types::pdu_version::PduVersionData;
use crate::utils::c_str;
use crate::utils::module_description::PduModuleDescription;
use crate::utils::root_file::Mvci;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Communication error: {0}")]
    CommError(#[from] PduError),

    #[error("FFI error: {0}")]
    FfiError(#[from] libloading::Error),
}

#[derive(Debug)]
pub struct Api {
    me: Weak<Api>,

    pdu_options: PduOptions,

    pdu_unique_id: PduUniqueId,

    library: libloading::Library,

    library_file: Option<PduLibraryPath>,

    module_description: Option<PduModuleDescription>,

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
        Ok(unsafe { self.library.get(name)? })
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

    pub fn pdu_get_version(&self, h_mod: u32, version_data: &mut VersionData) -> Result<PduVersionData> {
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

    pub fn pdu_get_object_id(&self, object: PduObjt, short_name: &str, use_mdf: Option<bool>) -> Result<u32> {
        const FUNC: &'static str = "PDUGetObjectId";
        self.log_api_call(FUNC);

        let use_mdf = use_mdf.unwrap_or(true);

        trace!(func = FUNC, object = object.as_ref(), short_name, use_mdf, "D-PDU API Call Args");

        if use_mdf {
            // First, we will try to obtain the required object ID from the module description
            // file supplied with the D-PDU API driver in order to reduce
            // the number of D-PDU API calls.
            let mdf_object_id = self.module_description
                .as_ref()
                .map(|desc| {
                    match object {
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
                    }
                })
                .flatten();

            if let Some(id) = mdf_object_id {
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
}
