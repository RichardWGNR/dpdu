use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::timing::{CpP2Min, CpP3Phys};
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub enum CpP3Func {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP3Func {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP3Func::Micros(v) => v.to_owned(),
            CpP3Func::Millis(v) => v.wrapping_mul(1000),
            CpP3Func::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP3Func> for ComParamDefinition {
    fn from(value: CpP3Func) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P3Func".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP3Func> for u32 {
    fn from(value: CpP3Func) -> Self {
        value.to_micros().into()
    }
}