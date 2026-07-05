pub mod pdu_event;
pub mod pdu_version;
pub mod pdu_object;
pub mod pdu_com_param;
pub mod pdu_com_param_table;
pub mod pdu_event_callback;
pub mod pdu_com_primivite;
pub mod pdu_io_ctl;
pub mod pdu_module;
pub mod pdu_status;
pub mod pdu_vci;
pub mod pdu_com_logical_link;
pub mod pdu_error_data;
pub mod pdu_resource;

use std::collections::HashMap;
use std::path::PathBuf;

pub type PduOptions = HashMap<String, String>;

pub type PduLibraryPath = PathBuf;

pub type PduUniqueId = u32;

pub type PduModuleHandle = u32;

pub type PduCllHandle = u32;

pub type PduCopHandle = u32;

pub type PduObjectId = u32;

pub type PduUniqueRespIdentifier = u32;