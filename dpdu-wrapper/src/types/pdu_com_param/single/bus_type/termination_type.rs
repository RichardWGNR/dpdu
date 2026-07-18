use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_TerminationType
///
/// Specifies the bus termination configuration. This parameter defines the
/// termination mode used for the CAN/CAN FD communication channel to ensure
/// proper signal integrity and minimize signal reflections on the bus. It is
/// used to configure the termination according to the physical network
/// topology.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpTerminationType(pub u32);

impl From<CpTerminationType> for ComParamDefinition {
    fn from(value: CpTerminationType) -> Self {
        ComParamDefinition {
            class: PduPc::BusType,
            short_name: "CP_TerminationType".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpTerminationType {
    pub const NO: Self = Self(0);
    pub const AC: Self = Self(1);
    pub const _60_OHM: Self = Self(2);
    pub const _120_OHM: Self = Self(3);
    pub const SWCAN: Self = Self(4);
}

impl From<CpTerminationType> for u32 {
    fn from(value: CpTerminationType) -> Self {
        value.0
    }
}

impl From<u32> for CpTerminationType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
