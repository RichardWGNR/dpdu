use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[derive(Debug, Copy, Clone)]
pub enum CpP2Max {
    Micros(u32),
    Millis(u32),
    Secs(u32)
}

impl CpP2Max {
    pub fn to_micros(&self) -> u32 {
        match self {
            CpP2Max::Micros(v) => v.to_owned(),
            CpP2Max::Millis(v) => v.wrapping_mul(1000),
            CpP2Max::Secs(v) => v.wrapping_mul(1000000)
        }
    }
}

impl From<CpP2Max> for ComParamDefinition {
    fn from(value: CpP2Max) -> Self {
        ComParamDefinition {
            class: PduPc::Timing,
            short_name: "CP_P2Max".to_string(),
            variant: value.to_micros().into(),
        }
    }
}

impl From<CpP2Max> for u32 {
    fn from(value: CpP2Max) -> Self {
        value.to_micros().into()
    }
}