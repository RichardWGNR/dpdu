use dpdu_api_types::{PduError, PDU_HANDLE_UNDEF};
use crate::dpdu::types::{PduCllHandle, PduCopHandle, PduModuleHandle};

#[derive(Debug, Clone)]
pub enum PduStatusTarget {
    System,
    Module(PduModuleHandle),
    LogicalLink(PduModuleHandle, PduCllHandle),
    Primitive(PduModuleHandle, PduCllHandle, PduCopHandle),
}

impl PduStatusTarget {
    pub fn from_api(h_mod: u32, h_cll: u32, h_cop: u32) -> Result<Self, PduError> {
        let h_mod = (h_mod != PDU_HANDLE_UNDEF).then(|| h_mod);
        let h_cll = (h_cll != PDU_HANDLE_UNDEF).then(|| h_cll);
        let h_cop = (h_cop != PDU_HANDLE_UNDEF).then(|| h_cop);
        
        match (h_mod, h_cll, h_cop) {
            (None, None, None) => Ok(PduStatusTarget::System),
            (Some(h_mod), None, None) => Ok(PduStatusTarget::Module(h_mod)),
            (Some(h_mod), Some(h_cll), None) => Ok(PduStatusTarget::LogicalLink(h_mod, h_cll)),
            (Some(h_mod), Some(h_cll), Some(h_cop)) => Ok(PduStatusTarget::Primitive(h_mod, h_cll, h_cop)),
            _ => Err(PduError::InvalidParameters)
        }
    }
    
    pub fn is_system(&self) -> bool {
        matches!(self, PduStatusTarget::System)
    }

    pub fn is_module(&self) -> bool {
        matches!(self, PduStatusTarget::Module(..))
    }

    pub fn is_logical_link(&self) -> bool {
        matches!(self, PduStatusTarget::LogicalLink(..))
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, PduStatusTarget::Primitive(..))
    }

    pub fn get_module_handle(&self) -> Option<PduModuleHandle> {
        match self {
            Self::Module(h_mod) => Some(h_mod.clone()),
            Self::LogicalLink(h_mod, ..) => Some(h_mod.clone()),
            Self::Primitive(h_mod, ..) => Some(h_mod.clone()),
            _ => None,
        }
    }

    pub fn get_cll_handle(&self) -> Option<PduCllHandle> {
        match self {
            Self::LogicalLink(_, h_cll, ..) => Some(h_cll.clone()),
            Self::Primitive(_, h_cll, ..) => Some(h_cll.clone()),
            _ => None,
        }
    }

    pub fn get_cop_handle(&self) -> Option<PduCopHandle> {
        match self {
            Self::Primitive(_, _, h_cop) => Some(h_cop.clone()),
            _ => None,
        }
    }
}