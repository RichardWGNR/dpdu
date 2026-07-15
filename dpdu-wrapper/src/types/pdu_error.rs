use crate::types::{PduCllHandle, PduCopHandle, PduModuleHandle};
use dpdu_api_types::PduErrorEvt;

#[derive(Debug, Clone)]
pub struct PduErrorData {
    pub target: PduLastErrorTarget,

    pub error_event: PduErrorEvt,

    pub h_cop: Option<PduCopHandle>,

    pub timestamp: u32,

    pub extra_info_code: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum PduLastErrorTarget {
    System,
    Module(PduModuleHandle),
    LogicalLink(PduModuleHandle, PduCllHandle),
}

impl PduLastErrorTarget {
    pub fn is_system(&self) -> bool {
        matches!(self, PduLastErrorTarget::System)
    }

    pub fn is_module(&self) -> bool {
        matches!(self, PduLastErrorTarget::Module(..))
    }

    pub fn is_logical_link(&self) -> bool {
        matches!(self, PduLastErrorTarget::LogicalLink(..))
    }

    pub fn get_module_handle(&self) -> Option<PduModuleHandle> {
        match self {
            PduLastErrorTarget::Module(h_mod) => Some(h_mod.clone()),
            PduLastErrorTarget::LogicalLink(h_mod, ..) => Some(h_mod.clone()),
            _ => None,
        }
    }

    pub fn get_cll_handle(&self) -> Option<PduCllHandle> {
        match self {
            PduLastErrorTarget::LogicalLink(_, h_cll) => Some(h_cll.clone()),
            _ => None,
        }
    }
}
