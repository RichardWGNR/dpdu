use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::PathBuf;

pub mod pdu_status;
pub mod pdu_event;

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