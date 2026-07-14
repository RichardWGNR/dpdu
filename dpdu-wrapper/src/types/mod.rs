pub mod pdu_com_logical_link;
pub mod pdu_com_param;
pub mod pdu_com_primitive;
pub mod pdu_error;
pub mod pdu_event;
pub mod pdu_io_ctl;
pub mod pdu_lock_resource;
pub mod pdu_module;
pub mod pdu_object;
pub mod pdu_resource;
pub mod pdu_status;
pub mod pdu_vci;
pub mod pdu_version;

use std::collections::HashMap;
use std::num::{NonZeroU32, NonZeroUsize};
use std::path::PathBuf;

pub type PduOptions = HashMap<String, String>;

pub type PduLibraryPath = PathBuf;

/// The value 0 represents a missing tag, so valid tags must be non-zero.
pub type PduUniqueApiTag = NonZeroUsize;

/// The value 0 represents a missing tag, so valid tags must be non-zero.
pub type PduUniqueCllTag = NonZeroUsize;

/// The value 0 represents a missing tag, so valid tags must be non-zero.
pub type PduUniqueCopTag = NonZeroUsize;

pub type PduModuleHandle = u32;

pub type PduCllHandle = u32;

pub type PduCopHandle = u32;

pub type PduObjectId = u32;

pub type PduUniqueRespIdentifier = u32;
