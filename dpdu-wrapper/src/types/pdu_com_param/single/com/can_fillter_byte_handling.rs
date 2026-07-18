use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpCanFillerByteHandling(pub bool);

impl From<CpCanFillerByteHandling> for ComParamDefinition {
    fn from(value: CpCanFillerByteHandling) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_CanFillerByteHandling".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl From<CpCanFillerByteHandling> for u32 {
    fn from(value: CpCanFillerByteHandling) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpCanFillerByteHandling> for bool {
    fn from(value: CpCanFillerByteHandling) -> Self {
        value.0
    }
}