use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::timing::{CpP2Min, CpP3Func};
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub enum CpP3Phys {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP3Phys {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP3Phys::Micros(v) => v.to_owned(),
            CpP3Phys::Millis(v) => v.wrapping_mul(1000),
            CpP3Phys::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP3Phys> for ComParamDefinition {
    fn from(value: CpP3Phys) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P3Phys".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP3Phys> for u32 {
    fn from(value: CpP3Phys) -> Self {
        value.to_micros().into()
    }
}