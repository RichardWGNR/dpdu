use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpCanRespUudtId(pub u32);

impl CpCanRespUudtId {
    pub const NOT_USED: CpCanRespUudtId = CpCanRespUudtId(0xFFFFFFFF);
}

impl From<CpCanRespUudtId> for ComParamDefinition {
    fn from(value: CpCanRespUudtId) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUUDTId".to_string(),
            variant: value.0.into(),
        }
    }
}

impl From<CpCanRespUudtId> for u32 {
    fn from(value: CpCanRespUudtId) -> Self {
        value.0
    }
}