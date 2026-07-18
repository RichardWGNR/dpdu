use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_SamplesPerBit
///
/// Specifies the number of samples taken during a single bit time. This
/// parameter defines whether the CAN controller uses single or multiple
/// sampling of the bus level to determine the transmitted bit value. It is
/// used to configure bit timing and improve communication reliability under
/// varying bus conditions.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpSamplesPerBit(pub u32);

impl From<CpSamplesPerBit> for ComParamDefinition {
    fn from(value: CpSamplesPerBit) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_SamplesPerBit".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpSamplesPerBit {
    pub const ONE_SAMPLE: Self = Self(0);
    pub const THREE_SAMPLE: Self = Self(0);
}

impl From<CpSamplesPerBit> for u32 {
    fn from(value: CpSamplesPerBit) -> Self {
        value.0
    }
}

impl From<u32> for CpSamplesPerBit {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
