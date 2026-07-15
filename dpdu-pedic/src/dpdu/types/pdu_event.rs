use dpdu_api_types::{PduError, PDU_HANDLE_UNDEF};
use crate::dpdu::types::{PduCllHandle, PduModuleHandle};

#[derive(Debug, Clone)]
pub enum PduEventTarget {
    System,
    Module(PduModuleHandle),
    LogicalLink(PduModuleHandle, PduCllHandle),
}

impl PduEventTarget {
    pub(crate) fn from_api(h_mod: PduModuleHandle, h_cll: PduCllHandle) -> Result<Self, PduError> {
        let h_mod_opt = (h_mod != PDU_HANDLE_UNDEF).then(|| h_mod);
        let h_cll_opt = (h_cll != PDU_HANDLE_UNDEF).then(|| h_cll);

        match (h_mod_opt, h_cll_opt) {
            (None, None) => Ok(PduEventTarget::System),
            (Some(h_mod), None) => Ok(PduEventTarget::Module(h_mod)),
            (Some(h_mod), Some(h_cll)) => Ok(PduEventTarget::LogicalLink(h_mod, h_cll)),
            _ => {
                Err(PduError::InvalidParameters)
            }
        }
    }

    pub fn is_system(&self) -> bool {
        matches!(self, PduEventTarget::System)
    }

    pub fn is_module(&self) -> bool {
        matches!(self, PduEventTarget::Module(..))
    }

    pub fn is_logical_link(&self) -> bool {
        matches!(self, PduEventTarget::LogicalLink(..))
    }

    pub fn get_module_handle(&self) -> Option<PduModuleHandle> {
        match self {
            PduEventTarget::Module(h_mod) => Some(h_mod.clone()),
            PduEventTarget::LogicalLink(h_mod, ..) => Some(h_mod.clone()),
            _ => None,
        }
    }

    pub fn get_cll_handle(&self) -> Option<PduCllHandle> {
        match self {
            PduEventTarget::LogicalLink(_, h_cll) => Some(h_cll.clone()),
            _ => None,
        }
    }
}