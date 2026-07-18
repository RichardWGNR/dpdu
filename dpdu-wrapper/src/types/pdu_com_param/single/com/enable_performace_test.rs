use crate::types::pdu_com_param::single::{deserialize_bool_from_u32, serialize_u32_from_bool};
use crate::types::pdu_com_param::table::ComParamDefinition;
use dpdu_api_types::PduPc;
use serde::{Deserialize, Serialize};

/// CP_EnablePerformanceTest
///
/// Specifies whether the performance measurement of the communication channel
/// between the diagnostic application and the ECU (or between the application
/// and the VCI) is enabled or disabled.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpEnablePerformanceTest(
    #[serde(deserialize_with = "deserialize_bool_from_u32")]
    #[serde(serialize_with = "serialize_u32_from_bool")]
    pub bool,
);

impl From<CpEnablePerformanceTest> for ComParamDefinition {
    fn from(value: CpEnablePerformanceTest) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_EnablePerformanceTest".to_string(),
            variant: value.0.then(|| 1u32).unwrap_or(0).into(),
        }
    }
}

impl CpEnablePerformanceTest {
    pub const DISABLE: Self = Self(false);
    pub const ENABLE: Self = Self(true);
}

impl From<CpEnablePerformanceTest> for u32 {
    fn from(value: CpEnablePerformanceTest) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpEnablePerformanceTest> for bool {
    fn from(value: CpEnablePerformanceTest) -> Self {
        value.0
    }
}

impl From<bool> for CpEnablePerformanceTest {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u32> for CpEnablePerformanceTest {
    fn from(value: u32) -> Self {
        Self(value > 0)
    }
}
