use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};
use crate::types::pdu_com_param::single::tester_present::CpTesterPresentMessage;
use crate::types::pdu_com_param::table::ComParamDefinition;

/// CP_CanBaudrateRecord
///
/// Specifies a CAN/CAN FD baud rate configuration record. This parameter
/// contains the communication timing configuration used to initialize the
/// CAN controller, including the nominal baud rate and, where applicable,
/// the data phase baud rate for CAN FD communication.
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpCanBaudrateRecord(pub Vec<u32>);

impl From<CpCanBaudrateRecord> for ComParamDefinition {
    fn from(value: CpCanBaudrateRecord) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_CanBaudrateRecord".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpCanBaudrateRecord {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl From<CpCanBaudrateRecord> for Vec<u32> {
    fn from(value: CpCanBaudrateRecord) -> Self {
        value.0
    }
}

impl From<Vec<u32>> for CpCanBaudrateRecord {
    fn from(value: Vec<u32>) -> Self {
        Self(value)
    }
}

impl From<&[u32]> for CpCanBaudrateRecord {
    fn from(value: &[u32]) -> Self {
        Self(value.to_vec())
    }
}