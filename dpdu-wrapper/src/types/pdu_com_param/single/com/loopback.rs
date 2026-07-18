use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpChangeEnablePerformanceTest;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub struct CpLoopback(pub bool);

impl From<CpLoopback> for ComParamDefinition {
    fn from(value: CpLoopback) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_Loopback".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl From<CpLoopback> for u32 {
    fn from(value: CpLoopback) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpLoopback> for bool {
    fn from(value: CpLoopback) -> Self {
        value.0
    }
}