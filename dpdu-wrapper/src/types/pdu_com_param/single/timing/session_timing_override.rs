use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::{ParamStructSessionTiming, PduPc};
use serde::{Deserialize, Serialize};

/// CP_SessionTimingOverride
///
/// Specifies whether the default diagnostic session timings shall be overridden
/// by user-defined values. This parameter indicates whether the system shall use
/// configured timer values instead of the values provided by the ECU during
/// diagnostic session establishment. It is used to configure communication
/// timing parameters when working with ECUs requiring non-standard timing
/// settings.
#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpSessionTimingOverride(pub Vec<ParamStructSessionTiming>);

impl From<CpSessionTimingOverride> for ComParamDefinition {
    fn from(value: CpSessionTimingOverride) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_SessionTimingOverride".to_string(),
            variant: value.0.into(),
        }
    }
}

impl CpSessionTimingOverride {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl From<CpSessionTimingOverride> for Vec<ParamStructSessionTiming> {
    fn from(value: CpSessionTimingOverride) -> Self {
        value.0
    }
}

impl From<&[ParamStructSessionTiming]> for CpSessionTimingOverride {
    fn from(value: &[ParamStructSessionTiming]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Vec<ParamStructSessionTiming>> for CpSessionTimingOverride {
    fn from(value: Vec<ParamStructSessionTiming>) -> Self {
        Self(value)
    }
}
