use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::timing::CpP2Max;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub enum CpP2Min {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP2Min {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP2Min::Micros(v) => v.to_owned(),
            CpP2Min::Millis(v) => v.wrapping_mul(1000),
            CpP2Min::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP2Min> for ComParamDefinition {
    fn from(value: CpP2Min) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P2Min".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP2Min> for u32 {
    fn from(value: CpP2Min) -> Self {
        value.to_micros().into()
    }
}