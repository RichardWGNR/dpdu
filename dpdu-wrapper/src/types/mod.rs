pub mod pdu_event;
pub mod pdu_version;

use std::collections::HashMap;
use std::path::PathBuf;

pub type PduOptions = HashMap<String, String>;

pub type PduLibraryPath = PathBuf;

pub type PduUniqueId = u32;

pub type PduModuleHandle = u32;

pub type PduCllHandle = u32;

pub type PduCopHandle = u32;