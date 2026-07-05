use dpdu_api_types::PduStatus;
use crate::types::{PduCllHandle, PduCopHandle, PduModuleHandle};

#[derive(Debug, Clone)]
pub struct PduStatusData {
    pub target: PduStatusTarget,
    pub status_code: PduStatus,
    pub timestamp: u32,
    pub extra_info: u32
}

#[derive(Debug, Clone)]
pub enum PduStatusTarget {
    System,
    Module(PduModuleHandle),
    ComLogicalLink(PduModuleHandle, PduCllHandle),
    ComPrimivite(PduModuleHandle, PduCllHandle, PduCopHandle)
}

impl PduStatusTarget {
    pub fn is_system(&self) -> bool {
        matches!(self, PduStatusTarget::System)
    }

    pub fn is_module(&self) -> bool {
        matches!(self, PduStatusTarget::Module(..))
    }

    pub fn is_com_logical_link(&self) -> bool {
        matches!(self, PduStatusTarget::ComLogicalLink(..))
    }

    pub fn is_com_primivite(&self) -> bool {
        matches!(self, PduStatusTarget::ComPrimivite(..))
    }

    pub fn get_module_handle(&self) -> Option<PduModuleHandle> {
        match self {
            Self::Module(h_mod) => Some(h_mod.clone()),
            Self::ComLogicalLink(h_mod, ..) => Some(h_mod.clone()),
            Self::ComPrimivite(h_mod, ..) => Some(h_mod.clone()),
            _ => None
        }
    }

    pub fn get_cll_handle(&self) -> Option<PduCllHandle> {
        match self {
            Self::ComLogicalLink(_, h_cll, ..) => Some(h_cll.clone()),
            Self::ComPrimivite(_, h_cll, ..) => Some(h_cll.clone()),
            _ => None
        }
    }

    pub fn get_cop_handle(&self) -> Option<PduCopHandle> {
        match self {
            Self::ComPrimivite(_, _, h_cop) => Some(h_cop.clone()),
            _ => None
        }
    }
}