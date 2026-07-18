use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpCanFillerByteHandling;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpChangeEnablePerformanceTest(pub bool);

impl From<CpChangeEnablePerformanceTest> for ComParamDefinition {
    fn from(value: CpChangeEnablePerformanceTest) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_EnablePerformanceTest".to_string(),
            variant: value.0.then(|| 1u32).unwrap_or(0).into()
        }
    }
}

impl From<CpChangeEnablePerformanceTest> for u32 {
    fn from(value: CpChangeEnablePerformanceTest) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpChangeEnablePerformanceTest> for bool {
    fn from(value: CpChangeEnablePerformanceTest) -> Self {
        value.0
    }
}