use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::unique::CpCanRespUudtExtAddr;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum CpCanRespUudtFormat {
    NormalUnsegmented11Bit = 0,
    NormalUnsegmented29Bit = 2,
    ExtUnsegmented11Bit = 8,
    ExtUnsegmented29Bit = 10
}

impl From<CpCanRespUudtFormat> for ComParamDefinition {
    fn from(value: CpCanRespUudtFormat) -> Self {
        ComParamDefinition {
            class: PduPc::UniqueId,
            short_name: "CP_CanRespUUDTFormat".to_string(),
            variant: (value as u32).into(),
        }
    }
}

impl From<CpCanRespUudtFormat> for u32 {
    fn from(value: CpCanRespUudtFormat) -> Self {
        value as _
    }
}