use dpdu_api_types::PduPc;
use crate::types::pdu_com_param::single::com::CpLoopback;
use crate::types::pdu_com_param::table::ComParamDefinition;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CpSendRemoteFrame(pub bool);

impl From<CpSendRemoteFrame> for ComParamDefinition {
    fn from(value: CpSendRemoteFrame) -> Self {
        ComParamDefinition {
            class: PduPc::Com,
            short_name: "CP_SendRemoteFrame".to_string(),
            variant: (if value.0 { 1 } else { 0 } as u32).into(),
        }
    }
}

impl From<CpSendRemoteFrame> for u32 {
    fn from(value: CpSendRemoteFrame) -> Self {
        if value.0 { 1 } else { 0 }
    }
}

impl From<CpSendRemoteFrame> for bool {
    fn from(value: CpSendRemoteFrame) -> Self {
        value.0
    }
}